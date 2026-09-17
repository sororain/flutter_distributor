use fastforge_core::{
    AppPublisher, PublishConfig, PublishError, PublishProgressCallback, PublishResult,
};
use std::env;
use std::process::Command;

pub struct FirebasePublisher;

const PUBLISHER_NAME: &str = "firebase";
const ENV_FIREBASE_TOKEN: &str = "FIREBASE_TOKEN";
const FIREBASE_CONSOLE_URL: &str = "https://console.firebase.google.com/project/_/appdistribution";

impl AppPublisher for FirebasePublisher {
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
        _on_progress: Option<&PublishProgressCallback>,
    ) -> Result<PublishResult, PublishError> {
        let token = env::var(ENV_FIREBASE_TOKEN)
            .map_err(|_| PublishError::MissingEnv(ENV_FIREBASE_TOKEN.to_string()))?;

        let artifact_path = config
            .artifact_path
            .as_deref()
            .ok_or_else(|| PublishError::MissingArgument("artifact_path".to_string()))?;

        let args = config.publish_arguments.as_ref();
        let app = args
            .and_then(|a| a.get("app"))
            .map(|s| s.as_str())
            .ok_or_else(|| PublishError::MissingArgument("app".to_string()))?;

        let mut cmd_args = vec![
            "appdistribution:distribute".to_string(),
            artifact_path.to_string(),
            "--app".to_string(),
            app.to_string(),
            "--token".to_string(),
            token,
        ];

        let optional_args = [
            "release-notes",
            "release-notes-file",
            "testers",
            "testers-file",
            "groups",
            "groups-file",
        ];
        for arg_name in &optional_args {
            if let Some(value) = args.and_then(|a| a.get(*arg_name))
                && !value.is_empty()
            {
                cmd_args.push(format!("--{arg_name}"));
                cmd_args.push(value.clone());
            }
        }

        let output = Command::new("firebase")
            .args(&cmd_args)
            .output()
            .map_err(|e| PublishError::CommandFailed(format!("Failed to run firebase CLI: {e}")))?;

        if output.status.success() {
            Ok(PublishResult {
                success: true,
                message: FIREBASE_CONSOLE_URL.to_string(),
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(PublishError::CommandFailed(format!(
                "{} - Upload to Firebase failed\n{}",
                output.status.code().unwrap_or(-1),
                stderr.trim()
            )))
        }
    }
}
