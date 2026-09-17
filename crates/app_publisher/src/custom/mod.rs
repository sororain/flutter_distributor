use fastforge_core::{
    AppPublisher, PublishConfig, PublishError, PublishProgressCallback, PublishResult,
};
use std::process::Command;

pub struct CustomPublisher;

const PUBLISHER_NAME: &str = "custom";

impl AppPublisher for CustomPublisher {
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
        let artifact_path = config
            .artifact_path
            .as_deref()
            .ok_or_else(|| PublishError::MissingArgument("artifact_path".to_string()))?;

        let args = config.publish_arguments.as_ref();
        let command = args
            .and_then(|a| a.get("command"))
            .filter(|v| !v.trim().is_empty())
            .ok_or_else(|| PublishError::MissingArgument("command".to_string()))?;

        let mut cmd = if cfg!(target_os = "windows") {
            let mut c = Command::new("cmd");
            c.args(["/C", command]);
            c
        } else {
            let mut c = Command::new("sh");
            c.args(["-c", command]);
            c
        };

        cmd.env("ARTIFACT_PATH", artifact_path);

        if let Some(pub_args) = args {
            for (key, value) in pub_args {
                if key != "command" {
                    let env_key = format!(
                        "PUBLISH_ARG_{}",
                        key.to_uppercase()
                            .chars()
                            .map(|c| if c.is_alphanumeric() { c } else { '_' })
                            .collect::<String>()
                    );
                    cmd.env(env_key, value);
                }
            }
        }

        let output = cmd.output().map_err(|e| {
            PublishError::CommandFailed(format!("Failed to run custom command: {e}"))
        })?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let message = stdout.trim().to_string();
            Ok(PublishResult {
                success: true,
                message: if message.is_empty() {
                    "Published successfully.".to_string()
                } else {
                    message
                },
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            Err(PublishError::CommandFailed(format!(
                "Custom command failed (exit code {})\n{}\n{}",
                output
                    .status
                    .code()
                    .map_or_else(|| "unknown".to_string(), |c| c.to_string()),
                stdout.trim(),
                stderr.trim()
            )))
        }
    }
}
