use fastforge_core::{
    AppPublisher, PublishConfig, PublishError, PublishProgressCallback, PublishResult,
};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::path::Path;

pub struct GitHubPublisher;

const PUBLISHER_NAME: &str = "github";
const ENV_GITHUB_TOKEN: &str = "GITHUB_TOKEN";
const ENV_GITHUB_REPOSITORY: &str = "GITHUB_REPOSITORY";
const GITHUB_API_BASE: &str = "https://api.github.com";
const GITHUB_ACCEPT: &str = "application/vnd.github+json";
const GITHUB_API_VERSION: &str = "2022-11-28";

#[derive(Debug, Deserialize)]
struct Release {
    upload_url: String,
}

#[derive(Debug, Deserialize)]
struct AssetResponse {
    browser_download_url: String,
}

impl AppPublisher for GitHubPublisher {
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

        let args = config.publish_arguments.as_ref();
        let token = env::var(ENV_GITHUB_TOKEN)
            .map_err(|_| PublishError::MissingEnv(ENV_GITHUB_TOKEN.to_string()))?;
        let repo = args
            .and_then(|a| a.get("repo").or_else(|| a.get("github-repo")))
            .cloned()
            .or_else(|| env::var(ENV_GITHUB_REPOSITORY).ok())
            .filter(|v| !v.trim().is_empty())
            .ok_or_else(|| PublishError::MissingArgument("repo".to_string()))?;

        let release_tag = args
            .and_then(|a| a.get("release-tag").or_else(|| a.get("github-release-tag")))
            .cloned()
            .or_else(|| config.app_version.clone());
        let release_title = args
            .and_then(|a| {
                a.get("release-title")
                    .or_else(|| a.get("github-release-title"))
            })
            .cloned();
        let draft = args
            .and_then(|a| {
                a.get("release-draft")
                    .or_else(|| a.get("github-release-draft"))
            })
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);
        let prerelease = args
            .and_then(|a| {
                a.get("release-prerelease")
                    .or_else(|| a.get("github-release-prerelease"))
            })
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        let client = Client::new();
        let release = find_or_create_release(
            &client,
            &token,
            &repo,
            release_tag.as_deref(),
            release_title.as_deref(),
            draft,
            prerelease,
        )?;
        let download_url = upload_asset(
            &client,
            &token,
            &release.upload_url,
            artifact_path,
            &file_name,
            on_progress,
        )?;

        Ok(PublishResult {
            success: true,
            message: download_url,
        })
    }
}

fn get_release_by_tag(
    client: &Client,
    token: &str,
    repo: &str,
    tag: &str,
) -> Result<Option<Release>, PublishError> {
    let url = format!("{GITHUB_API_BASE}/repos/{repo}/releases/tags/{tag}");
    let response = client
        .get(&url)
        .header(AUTHORIZATION, format!("token {token}"))
        .header(ACCEPT, GITHUB_ACCEPT)
        .header("X-GitHub-Api-Version", GITHUB_API_VERSION)
        .header(USER_AGENT, "fastforge")
        .send()
        .map_err(to_publish_error)?;
    if response.status().as_u16() == 404 {
        return Ok(None);
    }
    if !response.status().is_success() {
        let text = response.text().unwrap_or_default();
        return Err(PublishError::HttpError(format!(
            "GitHub get release by tag failed: {text}"
        )));
    }
    Ok(Some(response.json::<Release>().map_err(to_publish_error)?))
}

fn create_release(
    client: &Client,
    token: &str,
    repo: &str,
    tag: &str,
    name: &str,
    draft: bool,
    prerelease: bool,
) -> Result<Release, PublishError> {
    let url = format!("{GITHUB_API_BASE}/repos/{repo}/releases");
    let body = json!({
        "tag_name": tag,
        "name": name,
        "draft": draft,
        "prerelease": prerelease,
    });
    let response = client
        .post(&url)
        .header(AUTHORIZATION, format!("token {token}"))
        .header(ACCEPT, GITHUB_ACCEPT)
        .header("X-GitHub-Api-Version", GITHUB_API_VERSION)
        .header(USER_AGENT, "fastforge")
        .json(&body)
        .send()
        .map_err(to_publish_error)?;
    if !response.status().is_success() {
        let text = response.text().unwrap_or_default();
        return Err(PublishError::HttpError(format!(
            "Failed to create GitHub release: {text}"
        )));
    }
    response.json::<Release>().map_err(to_publish_error)
}

fn get_latest_release(client: &Client, token: &str, repo: &str) -> Result<Release, PublishError> {
    let url = format!("{GITHUB_API_BASE}/repos/{repo}/releases/latest");
    let response = client
        .get(&url)
        .header(AUTHORIZATION, format!("token {token}"))
        .header(ACCEPT, GITHUB_ACCEPT)
        .header("X-GitHub-Api-Version", GITHUB_API_VERSION)
        .header(USER_AGENT, "fastforge")
        .send()
        .map_err(to_publish_error)?;
    if !response.status().is_success() {
        let text = response.text().unwrap_or_default();
        return Err(PublishError::HttpError(format!(
            "Failed to get latest GitHub release: {text}"
        )));
    }
    response.json::<Release>().map_err(to_publish_error)
}

fn find_or_create_release(
    client: &Client,
    token: &str,
    repo: &str,
    tag: Option<&str>,
    title: Option<&str>,
    draft: bool,
    prerelease: bool,
) -> Result<Release, PublishError> {
    match tag {
        Some(tag) => {
            if let Some(release) = get_release_by_tag(client, token, repo, tag)? {
                return Ok(release);
            }
            let name = title.unwrap_or(tag);
            create_release(client, token, repo, tag, name, draft, prerelease)
        }
        None => get_latest_release(client, token, repo),
    }
}

fn upload_asset(
    client: &Client,
    token: &str,
    upload_url: &str,
    artifact_path: &str,
    file_name: &str,
    on_progress: Option<&PublishProgressCallback>,
) -> Result<String, PublishError> {
    let base_url = upload_url.split('{').next().unwrap_or(upload_url);
    let encoded_name = utf8_percent_encode(file_name, NON_ALPHANUMERIC).to_string();
    let url = format!("{base_url}?name={encoded_name}");

    let file = File::open(artifact_path).map_err(to_publish_error)?;
    let total_size = file.metadata().map_err(to_publish_error)?.len();
    let reader = UploadProgressReader::new(file, total_size, on_progress.cloned());

    let response = client
        .post(&url)
        .header(AUTHORIZATION, format!("token {token}"))
        .header(ACCEPT, GITHUB_ACCEPT)
        .header("X-GitHub-Api-Version", GITHUB_API_VERSION)
        .header(USER_AGENT, "fastforge")
        .header(CONTENT_TYPE, "application/octet-stream")
        .body(reqwest::blocking::Body::sized(reader, total_size))
        .send()
        .map_err(to_publish_error)?;

    if !response.status().is_success() {
        let text = response.text().unwrap_or_default();
        return Err(PublishError::HttpError(format!(
            "GitHub asset upload failed: {text}"
        )));
    }

    let asset: AssetResponse = response.json().map_err(to_publish_error)?;
    Ok(asset.browser_download_url)
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
