use std::process::Command;

use fastforge_core::{AppPackager, PackageConfig, PackageError, PackageResult, Platform};

/// Zips a Windows flutter build output directory using the `zip` CLI (Git Bash /
/// MSYS2) or PowerShell `Compress-Archive`, mirroring the Windows branch of
/// Dart's `AppPackageMakerZip`.
pub struct WindowsZipPackager;

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

impl AppPackager for WindowsZipPackager {
    fn name(&self) -> &str {
        "zip"
    }

    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn package_format(&self) -> &str {
        "zip"
    }

    fn package(&self, config: &PackageConfig) -> Result<PackageResult, PackageError> {
        let output_file = config.output_file();

        // Try `zip` first (available in Git Bash / MSYS2), fall back to PowerShell
        let zip_result = Command::new("zip")
            .args(["-r", &output_file.display().to_string(), "."])
            .current_dir(&config.build_output_dir)
            .output();

        if let Ok(out) = zip_result
            && out.status.success()
        {
            return Ok(PackageResult {
                artifacts: vec![output_file],
            });
        }

        // PowerShell fallback: Compress-Archive (paths passed as separate arguments)
        let src_glob = format!("{}\\*", config.build_output_dir.display());
        let dst_path = output_file.display().to_string();
        run(Command::new("powershell").args([
            "-NoProfile",
            "-Command",
            "Compress-Archive",
            "-Path",
            &src_glob,
            "-DestinationPath",
            &dst_path,
        ]))?;

        Ok(PackageResult {
            artifacts: vec![output_file],
        })
    }
}
