use serde_json::Value;
use thiserror::Error;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AnalyzeConfig {
    pub path: String,
}

impl AnalyzeConfig {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

#[derive(Debug)]
pub struct AnalyzeResult {
    pub success: bool,
    pub data: Value,
}

impl AnalyzeResult {
    pub fn new(success: bool, data: Value) -> Self {
        Self { success, data }
    }
}

#[derive(Debug, Error)]
pub enum AnalyzeError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Command '{command}' failed: {stderr}")]
    CommandFailed { command: String, stderr: String },
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    General(String),
}

// ── Trait ─────────────────────────────────────────────────────────────────────

pub trait AppAnalyzer {
    fn new() -> Self;
    fn name(&self) -> &str;
    fn is_supported_on_current_platform(&self) -> bool;

    fn perform_analyze(&self, config: &AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError>;

    fn analyze(&self, config: AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError> {
        self.perform_analyze(&config)
    }
}
