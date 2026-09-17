use fastforge_core::{
    AppPublisher, PublishConfig, PublishError, PublishProgressCallback, PublishResult,
};
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::path::Path;

pub struct AppGalleryPublisher;

const PUBLISHER_NAME: &str = "appgallery";
const ENV_CLIENT_ID: &str = "APP_GALLERY_CLIENT_ID";
const ENV_CLIENT_SECRET: &str = "APP_GALLERY_CLIENT_SECRET";
const BASE_URL: &str = "https://connect-api.cloud.huawei.com";

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct RetCode {
    code: i64,
    msg: String,
}

#[derive(Debug, Deserialize)]
struct UploadHeader {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct UploadUrlInfo {
    url: String,
    #[serde(rename = "objectId")]
    object_id: String,
    headers: Vec<UploadHeader>,
}

#[derive(Debug, Deserialize)]
struct UploadUrlResponse {
    ret: RetCode,
    #[serde(rename = "urlInfo")]
    url_info: Option<UploadUrlInfo>,
}

#[derive(Debug, Deserialize)]
struct ApplyResponse {
    ret: RetCode,
}

impl AppPublisher for AppGalleryPublisher {
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
        let artifact_path = config
            .artifact_path
            .as_deref()
            .ok_or_else(|| PublishError::MissingArgument("artifact_path".to_string()))?;
        let file_name = Path::new(artifact_path)
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| {
                PublishError::General("Cannot infer file name from artifact path.".to_string())
            })?
            .to_string();

        let client_id = optional_arg(config, "client-id")
            .or_else(|| env::var(ENV_CLIENT_ID).ok())
            .filter(|v| !v.trim().is_empty())
            .ok_or_else(|| PublishError::MissingArgument(ENV_CLIENT_ID.to_string()))?;
        let client_secret = optional_arg(config, "client-secret")
            .or_else(|| env::var(ENV_CLIENT_SECRET).ok())
            .filter(|v| !v.trim().is_empty())
            .ok_or_else(|| PublishError::MissingArgument(ENV_CLIENT_SECRET.to_string()))?;
        let app_id = optional_arg(config, "app-id")
            .filter(|v| !v.trim().is_empty())
            .ok_or_else(|| PublishError::MissingArgument("app-id".to_string()))?;

        let client = Client::new();
        let token = get_access_token(&client, &client_id, &client_secret)?;
        let file_size = std::fs::metadata(artifact_path)
            .map_err(to_publish_error)?
            .len();
        let url_info = get_upload_url(&client, &client_id, &token, &app_id, &file_name, file_size)?;
        upload_file(&client, artifact_path, &url_info, file_size, on_progress)?;
        apply_upload(
            &client,
            &client_id,
            &token,
            &app_id,
            &file_name,
            &url_info.object_id,
        )?;

        Ok(PublishResult {
            success: true,
            message:
                "https://developer.huawei.com/consumer/en/service/josp/agc/index.html#/appGallery"
                    .to_string(),
        })
    }
}

fn get_access_token(
    client: &Client,
    client_id: &str,
    client_secret: &str,
) -> Result<String, PublishError> {
    let url = format!("{BASE_URL}/api/oauth2/v1/token");
    let body = json!({
        "grant_type": "client_credentials",
        "client_id": client_id,
        "client_secret": client_secret,
    });
    let response = client
        .post(&url)
        .header(CONTENT_TYPE, "application/json")
        .json(&body)
        .send()
        .map_err(to_publish_error)?;
    if !response.status().is_success() {
        return Err(PublishError::HttpError(format!(
            "AppGallery token request failed with status {}",
            response.status()
        )));
    }
    let resp: TokenResponse = response.json().map_err(to_publish_error)?;
    Ok(resp.access_token)
}

fn get_upload_url(
    client: &Client,
    client_id: &str,
    token: &str,
    app_id: &str,
    file_name: &str,
    content_length: u64,
) -> Result<UploadUrlInfo, PublishError> {
    let url = format!("{BASE_URL}/api/publish/v2/upload-url/for-obs");
    let response = client
        .get(&url)
        .header("client_id", client_id)
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .header(CONTENT_TYPE, "application/json")
        .query(&[
            ("appId", app_id),
            ("fileName", file_name),
            ("contentLength", &content_length.to_string()),
        ])
        .send()
        .map_err(to_publish_error)?;
    if !response.status().is_success() {
        return Err(PublishError::HttpError(format!(
            "AppGallery upload-url request failed with status {}",
            response.status()
        )));
    }
    let resp: UploadUrlResponse = response.json().map_err(to_publish_error)?;
    if resp.ret.code != 0 {
        return Err(PublishError::ApiError {
            status: resp.ret.code.to_string(),
            message: resp.ret.msg,
        });
    }
    resp.url_info.ok_or_else(|| {
        PublishError::General("AppGallery upload-url response missing urlInfo.".to_string())
    })
}

fn upload_file(
    client: &Client,
    artifact_path: &str,
    url_info: &UploadUrlInfo,
    total_size: u64,
    on_progress: Option<&PublishProgressCallback>,
) -> Result<(), PublishError> {
    let file = File::open(artifact_path).map_err(to_publish_error)?;
    let reader = UploadProgressReader::new(file, total_size, on_progress.cloned());
    let mut request = client
        .put(&url_info.url)
        .body(reqwest::blocking::Body::sized(reader, total_size));
    for h in &url_info.headers {
        request = request.header(&h.name, &h.value);
    }
    let response = request.send().map_err(to_publish_error)?;
    if !response.status().is_success() {
        return Err(PublishError::HttpError(format!(
            "AppGallery file upload failed with status {}",
            response.status()
        )));
    }
    Ok(())
}

fn apply_upload(
    client: &Client,
    client_id: &str,
    token: &str,
    app_id: &str,
    file_name: &str,
    object_id: &str,
) -> Result<(), PublishError> {
    let url = format!("{BASE_URL}/api/publish/v3/app-package-info");
    let body = json!({
        "fileName": file_name,
        "objectId": object_id,
    });
    let response = client
        .put(&url)
        .header("client_id", client_id)
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .header(CONTENT_TYPE, "application/json")
        .query(&[
            ("appId", app_id),
            ("releaseType", "1"),
            ("releasePhase", "0"),
        ])
        .json(&body)
        .send()
        .map_err(to_publish_error)?;
    if !response.status().is_success() {
        return Err(PublishError::HttpError(format!(
            "AppGallery apply-upload request failed with status {}",
            response.status()
        )));
    }
    let resp: ApplyResponse = response.json().map_err(to_publish_error)?;
    if resp.ret.code != 0 {
        return Err(PublishError::ApiError {
            status: resp.ret.code.to_string(),
            message: resp.ret.msg,
        });
    }
    Ok(())
}

fn optional_arg(config: &PublishConfig, key: &str) -> Option<String> {
    config.publish_arguments.as_ref()?.get(key).cloned()
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
        if let Some(cb) = &on_progress {
            cb(0, total);
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
        let n = self.file.read(buf)?;
        if n > 0 {
            self.sent += n as u64;
            if let Some(cb) = &self.on_progress {
                cb(self.sent, self.total);
            }
        }
        Ok(n)
    }
}
