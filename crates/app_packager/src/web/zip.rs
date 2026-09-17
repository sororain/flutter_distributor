use std::process::Command;

use fastforge_core::{AppPackager, PackageConfig, PackageError, PackageResult, Platform};

/// Zips a flutter web build output directory, mirroring the web branch of
/// Dart's `AppPackageMakerZip`.
pub struct WebZipPackager;

fn run(cmd: &mut Command) -> Result<(), PackageError> {
    let out = cmd.output().map_err(|e| {
        PackageError::MissingTool(format!("{}: {}", cmd.get_program().to_string_lossy(), e))
    })?;
    if !out.status.success() {
        return Err(PackageError::CommandFailed {
            command: cmd.get_program().to_string_lossy().into(),
            stderr: String::from_utf8_lossy(&out.stderr).into(),
        });
    }
    Ok(())
}

impl AppPackager for WebZipPackager {
    fn name(&self) -> &str {
        "zip"
    }

    fn platform(&self) -> Platform {
        Platform::Web
    }

    fn package_format(&self) -> &str {
        "zip"
    }

    fn package(&self, config: &PackageConfig) -> Result<PackageResult, PackageError> {
        let output_file = config.output_file();
        run(Command::new("zip")
            .args(["-r", &output_file.display().to_string(), "."])
            .current_dir(&config.build_output_dir))?;
        Ok(PackageResult {
            artifacts: vec![output_file],
        })
    }
}
