use fastforge_core::{BuildConfig, BuildError};
use std::process::Command;

/// Pull a mandatory string argument from the config map.
pub fn extract_str<'a>(config: &'a BuildConfig, key: &str) -> Result<&'a str, BuildError> {
    config
        .arguments
        .get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            BuildError::InvalidArgument(format!(
                "Missing required argument `{}`. \
                 Provide a string value for the Xcode project.",
                key
            ))
        })
}

/// Pull an optional string argument from the config map.
pub fn extract_optional_str<'a>(config: &'a BuildConfig, key: &str) -> Option<&'a str> {
    config.arguments.get(key).and_then(|v| v.as_str())
}

/// Run a command with raw log output and check the exit status.
///
/// Unlike the captured-output variant, this inherits stdout/stderr so the
/// user sees raw progress from long-running tools like `xcodebuild`.
pub fn run(cmd: &mut Command) -> Result<(), BuildError> {
    let status = cmd
        .status()
        .map_err(|e| BuildError::Io(format!("{}: {}", cmd.get_program().to_string_lossy(), e)))?;
    if !status.success() {
        return Err(BuildError::CommandFailed(format!(
            "'{}' failed with exit code {}",
            cmd.get_program().to_string_lossy(),
            status.code().unwrap_or(-1),
        )));
    }
    Ok(())
}
