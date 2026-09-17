use fastforge_core::{
    AppPublisher, PublishConfig, PublishError, PublishProgressCallback, PublishResult,
};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct AppStorePublisher;

const PUBLISHER_NAME: &str = "appstore";
const ENV_APPSTORE_USERNAME: &str = "APPSTORE_USERNAME";
const ENV_APPSTORE_PASSWORD: &str = "APPSTORE_PASSWORD";
const ENV_APPSTORE_API_KEY: &str = "APPSTORE_APIKEY";
const ENV_APPSTORE_API_ISSUER: &str = "APPSTORE_APIISSUER";
const KEY_ID_ENV: &str = "APP_STORE_CONNECT_KEY_ID";
const ISSUER_ID_ENV: &str = "APP_STORE_CONNECT_ISSUER_ID";
const KEY_PATH_ENV: &str = "APP_STORE_CONNECT_KEY_PATH";
const APPSTORE_CONNECT_APPS_URL: &str = "https://appstoreconnect.apple.com/apps";

impl AppPublisher for AppStorePublisher {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &str {
        PUBLISHER_NAME
    }

    fn is_supported_on_current_platform(&self) -> bool {
        cfg!(target_os = "macos")
    }

    fn perform_publish(
        &self,
        config: &PublishConfig,
        _on_progress: Option<&PublishProgressCallback>,
    ) -> Result<PublishResult, PublishError> {
        if !self.is_supported_on_current_platform() {
            return Err(PublishError::General(
                "AppStore publisher is only supported on macOS.".to_string(),
            ));
        }

        let artifact_path = config
            .artifact_path
            .as_deref()
            .ok_or_else(|| PublishError::MissingArgument("artifact_path".to_string()))?;
        let artifact_path = std::fs::canonicalize(artifact_path).map_err(|error| {
            PublishError::General(format!(
                "Artifact path does not exist or cannot be resolved: {artifact_path}: {error}"
            ))
        })?;
        let artifact_type = appstore_artifact_type(&artifact_path)?;
        let auth = AppStoreAuth::from_config(config)?;

        let mut command = Command::new("xcrun");
        let mut args = vec![
            "altool".to_string(),
            "--upload-app".to_string(),
            "--file".to_string(),
            artifact_path.display().to_string(),
            "--type".to_string(),
            artifact_type.to_string(),
        ];
        args.extend(auth.to_cli_args());

        let staged_key = auth.stage_key_file()?;
        if let Some(staged_key) = &staged_key {
            command.current_dir(&staged_key.work_dir);
        }

        let output = command.args(args).output().map_err(to_publish_error)?;
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        if !output.status.success() {
            return Err(PublishError::CommandFailed(format!(
                "Upload of appstore failed: exit_code={:?}, stdout={}, stderr={}",
                output.status.code(),
                stdout,
                stderr
            )));
        }

        print_command_output(&stdout, &stderr);

        Ok(PublishResult {
            success: true,
            message: APPSTORE_CONNECT_APPS_URL.to_string(),
        })
    }
}

struct StagedKeyFile {
    work_dir: PathBuf,
}

impl Drop for StagedKeyFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.work_dir);
    }
}

struct AppStoreAuth {
    username: Option<String>,
    password: Option<String>,
    key_id: Option<String>,
    issuer_id: Option<String>,
    key_path: Option<String>,
}

impl AppStoreAuth {
    fn from_config(config: &PublishConfig) -> Result<Self, PublishError> {
        let username = optional_value(config, &["username"], &[ENV_APPSTORE_USERNAME]);
        let password = optional_value(config, &["password"], &[ENV_APPSTORE_PASSWORD]);
        let key_id = optional_value(
            config,
            &["key-id", "api-key"],
            &[KEY_ID_ENV, ENV_APPSTORE_API_KEY],
        );
        let issuer_id = optional_value(
            config,
            &["issuer-id", "api-issuer"],
            &[ISSUER_ID_ENV, ENV_APPSTORE_API_ISSUER],
        );
        let key_path = optional_value(config, &["key-path"], &[KEY_PATH_ENV]);

        let has_userpass = is_non_empty(&username) || is_non_empty(&password);
        let has_api = is_non_empty(&key_id) || is_non_empty(&issuer_id) || is_non_empty(&key_path);

        if !has_userpass && !has_api {
            return Err(PublishError::MissingEnv(format!(
                "`{ENV_APPSTORE_USERNAME}` & `{ENV_APPSTORE_PASSWORD}` or `{KEY_ID_ENV}` & `{ISSUER_ID_ENV}` & `{KEY_PATH_ENV}`"
            )));
        }
        if is_non_empty(&username) ^ is_non_empty(&password) {
            return Err(PublishError::MissingEnv(format!(
                "`{ENV_APPSTORE_USERNAME}` & `{ENV_APPSTORE_PASSWORD}`"
            )));
        }
        if has_api
            && !(is_non_empty(&key_id) && is_non_empty(&issuer_id) && is_non_empty(&key_path))
        {
            return Err(PublishError::MissingEnv(format!(
                "`{KEY_ID_ENV}` & `{ISSUER_ID_ENV}` & `{KEY_PATH_ENV}`"
            )));
        }

        Ok(Self {
            username,
            password,
            key_id,
            issuer_id,
            key_path,
        })
    }

    fn to_cli_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        push_flag(&mut args, "--username", self.username.as_deref());
        push_flag(&mut args, "--password", self.password.as_deref());
        push_flag(&mut args, "--apiKey", self.key_id.as_deref());
        push_flag(&mut args, "--apiIssuer", self.issuer_id.as_deref());
        args
    }

    fn stage_key_file(&self) -> Result<Option<StagedKeyFile>, PublishError> {
        let (Some(key_id), Some(key_path)) = (&self.key_id, &self.key_path) else {
            return Ok(None);
        };

        let source = expand_tilde(key_path);
        if !source.is_file() {
            return Err(PublishError::General(format!(
                "{KEY_PATH_ENV} does not point to a readable file: {}",
                source.display()
            )));
        }

        let work_dir = env::temp_dir().join(format!(
            "fastforge-appstore-{}-{}",
            std::process::id(),
            unix_timestamp_millis()
        ));
        let key_dir = work_dir.join("private_keys");
        std::fs::create_dir_all(&key_dir).map_err(to_publish_error)?;

        let destination = key_dir.join(format!("AuthKey_{key_id}.p8"));
        std::fs::copy(&source, &destination).map_err(to_publish_error)?;

        Ok(Some(StagedKeyFile { work_dir }))
    }
}

fn appstore_artifact_type(path: &Path) -> Result<&'static str, PublishError> {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| {
            PublishError::General(format!(
                "Cannot infer artifact type from file path: {}",
                path.display()
            ))
        })?;
    match extension {
        "ipa" => Ok("ios"),
        "pkg" => Ok("osx"),
        _ => Ok("osx"),
    }
}

fn optional_value(
    config: &PublishConfig,
    argument_keys: &[&str],
    env_keys: &[&str],
) -> Option<String> {
    config
        .publish_arguments
        .as_ref()
        .and_then(|arguments| {
            argument_keys
                .iter()
                .find_map(|key| arguments.get(*key).cloned())
        })
        .or_else(|| env_keys.iter().find_map(|key| env::var(key).ok()))
        .filter(|value| !value.trim().is_empty())
}

fn is_non_empty(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|v| !v.trim().is_empty())
}

fn push_flag(args: &mut Vec<String>, flag: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|v| !v.trim().is_empty()) {
        args.push(flag.to_string());
        args.push(value.to_string());
    }
}

fn expand_tilde(path: &str) -> PathBuf {
    if path == "~" {
        return env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(path));
    }
    if let Some(rest) = path.strip_prefix("~/")
        && let Some(home) = env::var_os("HOME")
    {
        return PathBuf::from(home).join(rest);
    }
    PathBuf::from(path)
}

fn unix_timestamp_millis() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}

fn print_command_output(stdout: &str, stderr: &str) {
    if !stdout.is_empty() {
        println!("{stdout}");
    }
    if !stderr.is_empty() {
        eprintln!("{stderr}");
    }
}

fn to_publish_error(error: impl std::fmt::Display) -> PublishError {
    PublishError::General(error.to_string())
}
