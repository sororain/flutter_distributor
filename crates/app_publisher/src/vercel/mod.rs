use fastforge_core::{
    AppPublisher, PublishConfig, PublishError, PublishProgressCallback, PublishResult,
};
use serde_json::json;
use std::env;
use std::fs;
use std::process::Command;

pub struct VercelPublisher;

const PUBLISHER_NAME: &str = "vercel";
const ENV_VERCEL_ORG_ID: &str = "VERCEL_ORG_ID";
const ENV_VERCEL_PROJECT_ID: &str = "VERCEL_PROJECT_ID";

impl AppPublisher for VercelPublisher {
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
        let org_id = args
            .and_then(|a| a.get("org-id").or_else(|| a.get("vercel-org-id")))
            .cloned()
            .or_else(|| env::var(ENV_VERCEL_ORG_ID).ok())
            .filter(|v| !v.trim().is_empty())
            .ok_or_else(|| PublishError::MissingArgument("org-id".to_string()))?;
        let project_id = args
            .and_then(|a| a.get("project-id").or_else(|| a.get("vercel-project-id")))
            .cloned()
            .or_else(|| env::var(ENV_VERCEL_PROJECT_ID).ok())
            .filter(|v| !v.trim().is_empty())
            .ok_or_else(|| PublishError::MissingArgument("project-id".to_string()))?;

        let vercel_dir = format!("{artifact_path}/.vercel");
        fs::create_dir_all(&vercel_dir).map_err(|e| {
            PublishError::General(format!("Failed to create .vercel directory: {e}"))
        })?;
        let project_json = json!({ "orgId": org_id, "projectId": project_id });
        fs::write(
            format!("{vercel_dir}/project.json"),
            serde_json::to_string_pretty(&project_json).unwrap(),
        )
        .map_err(|e| PublishError::General(format!("Failed to write .vercel/project.json: {e}")))?;

        let output = Command::new("vercel")
            .arg("--prod")
            .current_dir(artifact_path)
            .output()
            .map_err(|e| PublishError::CommandFailed(format!("Failed to run vercel CLI: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(PublishError::CommandFailed(format!(
                "vercel deploy failed ({})\n{}\n{}",
                output.status.code().unwrap_or(-1),
                stdout.trim(),
                stderr.trim()
            )));
        }

        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let url = format!("{stderr}\n{stdout}")
            .lines()
            .find_map(|line| {
                let line = line.trim();
                line.find("Production: ").map(|idx| {
                    let rest = &line[idx + "Production: ".len()..];
                    rest.split_whitespace()
                        .next()
                        .unwrap_or("https://vercel.com")
                        .to_string()
                })
            })
            .unwrap_or_else(|| "https://vercel.com".to_string());

        Ok(PublishResult {
            success: true,
            message: url,
        })
    }
}
