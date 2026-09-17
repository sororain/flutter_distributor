use fastforge_core::{
    AppPublisher, PublishConfig, PublishError, PublishProgressCallback, PublishResult,
};
use serde_json::json;
use std::env;
use std::fs;
use std::process::Command;

pub struct FirebaseHostingPublisher;

const PUBLISHER_NAME: &str = "firebase-hosting";
const ENV_FIREBASE_TOKEN: &str = "FIREBASE_TOKEN";
const ENV_FIREBASE_PROJECT_ID: &str = "FIREBASE_PROJECT_ID";

impl AppPublisher for FirebaseHostingPublisher {
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
        let project_id = args
            .and_then(|a| a.get("project-id").or_else(|| a.get("firebase-project-id")))
            .cloned()
            .or_else(|| env::var(ENV_FIREBASE_PROJECT_ID).ok())
            .filter(|v| !v.trim().is_empty())
            .ok_or_else(|| PublishError::MissingArgument("project-id".to_string()))?;

        let firebaserc = json!({ "projects": { "default": project_id } });
        fs::write(
            format!("{artifact_path}/.firebaserc"),
            serde_json::to_string_pretty(&firebaserc).unwrap(),
        )
        .map_err(|e| PublishError::General(format!("Failed to write .firebaserc: {e}")))?;

        let firebase_json = json!({
            "hosting": { "public": ".", "ignore": ["firebase.json"] }
        });
        fs::write(
            format!("{artifact_path}/firebase.json"),
            serde_json::to_string_pretty(&firebase_json).unwrap(),
        )
        .map_err(|e| PublishError::General(format!("Failed to write firebase.json: {e}")))?;

        let mut cmd = Command::new("firebase");
        cmd.arg("deploy").current_dir(artifact_path);
        if let Ok(token) = env::var(ENV_FIREBASE_TOKEN) {
            cmd.arg("--token").arg(token);
        }

        let output = cmd
            .output()
            .map_err(|e| PublishError::CommandFailed(format!("Failed to run firebase CLI: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PublishError::CommandFailed(format!(
                "firebase deploy failed ({})\n{}",
                output.status.code().unwrap_or(-1),
                stderr.trim()
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let url = stdout
            .lines()
            .find_map(|line| {
                let line = line.trim();
                line.find("Hosting URL: ").map(|idx| {
                    let rest = &line[idx + "Hosting URL: ".len()..];
                    rest.split_whitespace()
                        .next()
                        .unwrap_or("https://firebase.google.com")
                        .to_string()
                })
            })
            .unwrap_or_else(|| "https://firebase.google.com".to_string());

        Ok(PublishResult {
            success: true,
            message: url,
        })
    }
}
