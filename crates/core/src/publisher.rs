use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

// ── Types ─────────────────────────────────────────────────────────────────────

pub type PublishProgressCallback = Arc<dyn Fn(u64, u64) + Send + Sync + 'static>;

#[derive(Debug, Clone)]
pub struct PublishConfig {
    pub app_version: Option<String>,
    pub artifact_path: Option<String>,
    pub publish_arguments: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct PublishResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum PublishError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(String),
    #[error("Missing publish argument '{0}'")]
    MissingArgument(String),
    #[error("HTTP request failed: {0}")]
    HttpError(String),
    #[error("API error: {status} {message}")]
    ApiError { status: String, message: String },
    #[error("Command failed: {0}")]
    CommandFailed(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    General(String),
}

// ── Trait ─────────────────────────────────────────────────────────────────────

pub trait AppPublisher {
    fn new() -> Self;
    fn name(&self) -> &str;
    fn is_supported_on_current_platform(&self) -> bool;

    fn perform_publish(
        &self,
        config: &PublishConfig,
        on_progress: Option<&PublishProgressCallback>,
    ) -> Result<PublishResult, PublishError>;

    fn publish(
        &self,
        config: PublishConfig,
        on_progress: Option<PublishProgressCallback>,
    ) -> Result<PublishResult, PublishError> {
        self.perform_publish(&config, on_progress.as_ref())
    }
}
