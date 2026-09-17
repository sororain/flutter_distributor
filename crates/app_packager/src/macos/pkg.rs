use std::path::Path;
use std::process::Command;

use fastforge_core::{AppPackager, PackageConfig, PackageError, PackageResult, Platform};
use serde::Deserialize;

/// Builds a macOS `.pkg` installer using `xcrun productbuild` (and optionally
/// `productsign`), mirroring Dart's `AppPackageMakerPkg`.
///
/// Requires Xcode command-line tools.
#[derive(Default)]
pub struct MacOSPkgPackager {
    /// Optional code-signing identity (e.g. `"Developer ID Installer: ..."`)
    pub sign_identity: Option<String>,
    /// Installation path prefix (defaults to `/Applications/`)
    pub install_path: Option<String>,
    /// Optional path to a directory containing pre/post-install scripts.
    pub scripts: Option<String>,
}

impl MacOSPkgPackager {
    /// Load configuration from a YAML file (e.g. `macos/packaging/pkg/make_config.yaml`).
    /// Returns `Self::default()` if the file does not exist.
    pub fn from_yaml_file(path: &Path) -> Result<Self, PackageError> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path).map_err(|e| {
            PackageError::General(format!("Failed to read {}: {}", path.display(), e))
        })?;
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct Config {
            install_path: Option<String>,
            sign_identity: Option<String>,
            scripts: Option<String>,
        }
        let cfg: Config = serde_yaml::from_str(&content).map_err(|e| {
            PackageError::General(format!("Failed to parse {}: {}", path.display(), e))
        })?;
        Ok(Self {
            sign_identity: cfg.sign_identity,
            install_path: cfg.install_path,
            scripts: cfg.scripts,
        })
    }
}

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

impl AppPackager for MacOSPkgPackager {
    fn name(&self) -> &str {
        "pkg"
    }

    fn platform(&self) -> Platform {
        Platform::MacOS
    }

    fn package_format(&self) -> &str {
        "pkg"
    }

    #[cfg(not(target_os = "macos"))]
    fn is_supported_on_current_platform(&self) -> bool {
        false
    }

    fn package(&self, config: &PackageConfig) -> Result<PackageResult, PackageError> {
        // Flutter's macOS build exposes the .app bundle as the first entry in
        // build_output_files. Use productbuild's component mode so the bundle
        // is installed as /Applications/<App>.app instead of flattening the
        // app bundle contents into /Applications.
        let app_path = config
            .first_build_output_file()
            .ok_or_else(|| PackageError::General("no build output files".into()))?;

        let output_file = config.output_file();
        let install_path = self.install_path.as_deref().unwrap_or("/Applications/");

        let unsigned_path = {
            let mut p = output_file.clone();
            let stem = p
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();
            p.set_file_name(format!("{}-unsigned.pkg", stem));
            p
        };

        let mut productbuild_args = vec![
            "productbuild".to_string(),
            "--component".to_string(),
            app_path.display().to_string(),
            install_path.to_string(),
        ];

        // Include installer scripts if configured
        if let Some(scripts_dir) = &self.scripts {
            let scripts_path = Path::new(scripts_dir);
            if scripts_path.exists() {
                productbuild_args.push("--scripts".to_string());
                productbuild_args.push(scripts_path.to_string_lossy().to_string());
            }
        }

        productbuild_args.push(unsigned_path.display().to_string());

        run(Command::new("xcrun").args(&productbuild_args))?;

        if let Some(identity) = &self.sign_identity {
            run(Command::new("xcrun").args([
                "productsign",
                "--sign",
                identity,
                &unsigned_path.display().to_string(),
                &output_file.display().to_string(),
            ]))?;
            std::fs::remove_file(&unsigned_path)?;
        } else {
            std::fs::rename(&unsigned_path, &output_file)?;
        }

        Ok(PackageResult {
            artifacts: vec![output_file],
        })
    }
}
