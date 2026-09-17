use fastforge_core::{
    AppPublisher, PublishConfig, PublishError, PublishProgressCallback, PublishResult,
};
use reqwest::blocking::Client;
use reqwest::blocking::multipart::{Form, Part};
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::path::Path;

pub struct FirPublisher;

const PUBLISHER_NAME: &str = "fir";
const ENV_FIR_API_TOKEN: &str = "FIR_API_TOKEN";
const FIR_APPS_URL: &str = "http://api.bq04.com/apps";

#[derive(Debug, Deserialize)]
struct FirAppData {
    cert: FirCert,
    download_domain: String,
    download_domain_https_ready: bool,
    short: String,
}

#[derive(Debug, Deserialize)]
struct FirCert {
    binary: FirBinaryCert,
}

#[derive(Debug, Deserialize)]
struct FirBinaryCert {
    key: String,
    token: String,
    upload_url: String,
}

#[derive(Debug, Deserialize)]
struct FirUploadData {
    release_id: String,
}

#[derive(Debug, Deserialize)]
struct FirErrorBody {
    code: Option<i64>,
    errors: Option<FirErrors>,
}

#[derive(Debug, Deserialize)]
struct FirErrors {
    exception: Option<Vec<String>>,
}

impl AppPublisher for FirPublisher {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &str {
        PUBLISHER_NAME
    }

    fn is_supported_on_current_platform(&self) -> bool {
        true
    }

    fn perform_publish(
        &self,
        config: &PublishConfig,
        on_progress: Option<&PublishProgressCallback>,
    ) -> Result<PublishResult, PublishError> {
        let api_token = env::var(ENV_FIR_API_TOKEN)
            .map_err(|_| PublishError::MissingEnv(ENV_FIR_API_TOKEN.to_string()))?;

        let artifact_path = config
            .artifact_path
            .as_deref()
            .ok_or_else(|| PublishError::MissingArgument("artifact_path".to_string()))?;

        let args = config.publish_arguments.as_ref();
        let bundle_id = args
            .and_then(|a| a.get("bundle_id"))
            .map(|s| s.as_str())
            .ok_or_else(|| PublishError::MissingArgument("bundle_id".to_string()))?;

        let platform = infer_platform(artifact_path).ok_or_else(|| {
            PublishError::General(format!(
                "Cannot infer platform from artifact path: {artifact_path}"
            ))
        })?;

        let app_name = args
            .and_then(|a| a.get("app_name"))
            .map(|s| s.as_str())
            .unwrap_or("");
        let version = args
            .and_then(|a| a.get("version"))
            .map(|s| s.as_str())
            .unwrap_or("");
        let build_number = args
            .and_then(|a| a.get("build_number"))
            .map(|s| s.as_str())
            .unwrap_or("");

        let client = Client::new();

        let app_data = self.get_upload_cert(&client, &api_token, platform, bundle_id)?;
        let release_id = self.upload_binary(
            &client,
            artifact_path,
            &app_data.cert.binary.key,
            &app_data.cert.binary.token,
            &app_data.cert.binary.upload_url,
            app_name,
            version,
            build_number,
            on_progress,
        )?;

        let scheme = if app_data.download_domain_https_ready {
            "https"
        } else {
            "http"
        };
        let url = format!(
            "{}://{}/{}?release_id={}",
            scheme, app_data.download_domain, app_data.short, release_id
        );

        Ok(PublishResult {
            success: true,
            message: url,
        })
    }
}

impl FirPublisher {
    fn get_upload_cert(
        &self,
        client: &Client,
        api_token: &str,
        platform: &str,
        bundle_id: &str,
    ) -> Result<FirAppData, PublishError> {
        let body = json!({
            "type": platform,
            "bundle_id": bundle_id,
            "api_token": api_token,
        });

        let response = client
            .post(FIR_APPS_URL)
            .json(&body)
            .send()
            .map_err(to_publish_error)?;

        if !response.status().is_success() {
            let err_body: Option<FirErrorBody> = response.json().ok();
            let message = err_body
                .as_ref()
                .and_then(|b| {
                    let code = b.code?;
                    let msg = b.errors.as_ref()?.exception.as_ref()?.first()?;
                    Some(format!("{code} - {msg}"))
                })
                .unwrap_or_else(|| "fir /apps request failed".to_string());
            return Err(PublishError::HttpError(message));
        }

        response.json::<FirAppData>().map_err(to_publish_error)
    }

    #[allow(clippy::too_many_arguments)]
    fn upload_binary(
        &self,
        client: &Client,
        artifact_path: &str,
        key: &str,
        token: &str,
        upload_url: &str,
        app_name: &str,
        version: &str,
        build_number: &str,
        on_progress: Option<&PublishProgressCallback>,
    ) -> Result<String, PublishError> {
        let file_name = file_name(artifact_path).ok_or_else(|| {
            PublishError::General(format!(
                "Cannot infer file name from artifact path: {artifact_path}"
            ))
        })?;
        let file = File::open(artifact_path).map_err(to_publish_error)?;
        let total_size = file.metadata().map_err(to_publish_error)?.len();
        let progress_reader = UploadProgressReader::new(file, total_size, on_progress.cloned());

        let form = Form::new()
            .text("key", key.to_string())
            .text("token", token.to_string())
            .text("x:name", app_name.to_string())
            .text("x:version", version.to_string())
            .text("x:build", build_number.to_string())
            .part(
                "file",
                Part::reader(progress_reader)
                    .file_name(file_name)
                    .mime_str("application/octet-stream")
                    .map_err(to_publish_error)?,
            );

        let response = client
            .post(upload_url)
            .multipart(form)
            .send()
            .map_err(to_publish_error)?;

        if !response.status().is_success() {
            return Err(PublishError::HttpError(format!(
                "fir upload failed with status: {}",
                response.status()
            )));
        }

        let data: FirUploadData = response.json().map_err(to_publish_error)?;
        Ok(data.release_id)
    }
}

fn infer_platform(path: &str) -> Option<&str> {
    match Path::new(path).extension().and_then(|e| e.to_str()) {
        Some("apk") => Some("android"),
        Some("ipa") => Some("ios"),
        _ => None,
    }
}

fn file_name(path: &str) -> Option<String> {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .map(ToOwned::to_owned)
}

fn to_publish_error(error: impl std::fmt::Display) -> PublishError {
    PublishError::General(error.to_string())
}

struct UploadProgressReader {
    file: File,
    sent: u64,
    total: u64,
    on_progress: Option<PublishProgressCallback>,
}

impl UploadProgressReader {
    fn new(file: File, total: u64, on_progress: Option<PublishProgressCallback>) -> Self {
        if let Some(callback) = &on_progress {
            callback(0, total);
        }
        Self {
            file,
            sent: 0,
            total,
            on_progress,
        }
    }
}

impl Read for UploadProgressReader {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let bytes_read = self.file.read(buf)?;
        if bytes_read > 0 {
            self.sent += bytes_read as u64;
            if let Some(callback) = &self.on_progress {
                callback(self.sent, self.total);
            }
        }
        Ok(bytes_read)
    }
}
