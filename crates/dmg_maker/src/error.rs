use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DmgMakerError {
    #[error("Platform not supported: {0}")]
    UnsupportedPlatform(String),

    #[error("Target already exists: {0}")]
    TargetExists(PathBuf),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Command failed: {command}\n{stderr}")]
    CommandFailed { command: String, stderr: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    General(String),
}
