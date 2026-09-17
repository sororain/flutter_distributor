use fastforge_core::{AppBuilder, BuildConfig, BuildError, BuildResult, Platform};
use glob::glob;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use super::helpers::{extract_optional_str, extract_str, run};

// ─── MacOSXcodeBuilder ─────────────────────────────────────────────────────────

/// A builder for native macOS Xcode projects.
///
/// Unlike the Flutter `MacOSBuilder`, this runs `xcodebuild` directly and
/// resolves the `.app` bundle from the Xcode build products directory.
///
/// # Minimum viable invocation
///
/// ```text
/// platform = "macos"
/// target   = "macos-xcode"  (distinct from Flutter's "macos")
/// arguments = {
///   "project":     "macos/Runner.xcodeproj",   // path to .xcodeproj
///   "scheme":      "Runner",                    // Xcode scheme name
///   "configuration": "Release",                 // Debug / Release / Profile
///   "derived-data-path": "macos/build",         // optional, defaults to Xcode default
///   "product-name": "ownCal",                   // the .app name (from AppInfo.xcconfig)
///   "xcconfig-override": "Runner/Configs/Release.xcconfig", // optional
/// }
/// ```
pub struct MacOSXcodeBuilder;

// ─── AppBuilder impl ──────────────────────────────────────────────────────────

impl AppBuilder for MacOSXcodeBuilder {
    fn name(&self) -> &str {
        "macos-xcode"
    }

    fn platform(&self) -> Platform {
        Platform::MacOS
    }

    fn matches(&self, platform: &Platform, target: Option<&str>) -> bool {
        platform == &Platform::MacOS && target == Some("macos-xcode")
    }

    fn is_supported_on_current_platform(&self) -> bool {
        Platform::current() == Some(Platform::MacOS)
    }

    fn build_subcommand(&self) -> &str {
        "macos-xcode"
    }

    fn validate_arguments(&self, config: &BuildConfig) -> Result<(), BuildError> {
        extract_str(config, "project")?;
        extract_str(config, "scheme")?;
        Ok(())
    }

    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let project = extract_str(config, "project")?;
        let _scheme = extract_str(config, "scheme")?;
        let configuration = extract_optional_str(config, "configuration").unwrap_or("Release");
        let product_name = extract_optional_str(config, "product-name");
        let derived_data_path = extract_optional_str(config, "derived-data-path");

        // Determine the build products directory. When `-derivedDataPath` is
        // passed to `xcodebuild`, it places build products under
        // `<derivedDataPath>/Build/Products/<configuration>/`, not directly
        // under `<derivedDataPath>/<configuration>/`.
        let products_dir: PathBuf = if let Some(ddp) = derived_data_path {
            PathBuf::from(ddp)
                .join("Build")
                .join("Products")
                .join(configuration)
        } else {
            let project_dir = std::path::Path::new(project)
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."));
            project_dir.join("build").join(configuration)
        };

        let app_pattern = if let Some(name) = product_name {
            format!("{}/{}.app", products_dir.display(), name)
        } else {
            format!("{}/*.app", products_dir.display())
        };

        let mut files: Vec<PathBuf> = glob(&app_pattern)
            .map_err(|e| BuildError::Parse(format!("Invalid glob pattern: {}", e)))?
            .filter_map(Result::ok)
            .collect();
        files.sort();
        files.dedup();

        Ok((products_dir, files))
    }

    fn build_result(
        &self,
        config: BuildConfig,
        output_directory: PathBuf,
        output_files: Vec<PathBuf>,
        duration_ms: u128,
    ) -> BuildResult {
        let target = config
            .arguments
            .get("target")
            .and_then(|v| v.as_str())
            .map(ToString::to_string);

        BuildResult {
            config,
            output_directory,
            output_files,
            duration_ms,
            platform: Platform::MacOS,
            target,
        }
    }
}

// ─── MacOSXcodeAppBuilder ────────────────────────────────────────────────────

/// High-level runner for native macOS Xcode projects.
///
/// Usage:
///
/// ```rust,ignore
/// let result = MacOSXcodeAppBuilder::default().build(
///     "macos",
///     Some("macos-xcode"),
///     args_map,
///     None,
/// )?;
/// ```
pub struct MacOSXcodeAppBuilder {
    builder: MacOSXcodeBuilder,
}

impl Default for MacOSXcodeAppBuilder {
    fn default() -> Self {
        Self {
            builder: MacOSXcodeBuilder,
        }
    }
}

impl MacOSXcodeAppBuilder {
    pub fn build(
        &self,
        platform: &str,
        _target: Option<&str>,
        arguments: serde_json::Map<String, serde_json::Value>,
        environment: Option<HashMap<String, String>>,
    ) -> Result<BuildResult, BuildError> {
        if platform != "macos" {
            return Err(BuildError::UnsupportedBuilder(format!(
                "MacOSXcodeAppBuilder only handles platform=\"macos\", got \"{}\"",
                platform
            )));
        }

        let config = BuildConfig::new(arguments);
        self.builder.validate_arguments(&config)?;

        let project = extract_str(&config, "project")?.to_string();
        let scheme = extract_str(&config, "scheme")?.to_string();
        let configuration = extract_optional_str(&config, "configuration")
            .unwrap_or("Release")
            .to_string();
        let derived_data_path = extract_optional_str(&config, "derived-data-path");
        let xcconfig = extract_optional_str(&config, "xcconfig-override");

        let mut cmd = Command::new("xcodebuild");
        cmd.args([
            "-project",
            &project,
            "-scheme",
            &scheme,
            "-configuration",
            &configuration,
        ]);

        if let Some(sdk) = extract_optional_str(&config, "sdk") {
            cmd.args(["-sdk", sdk]);
        }

        if let Some(ddp) = derived_data_path {
            cmd.args(["-derivedDataPath", ddp]);
        }

        if let Some(xc) = xcconfig {
            cmd.args(["-xcconfig", xc]);
        }

        if let Some(flags) = config
            .arguments
            .get("extra-flags")
            .and_then(|v| v.as_array())
        {
            for flag in flags {
                if let Some(f) = flag.as_str() {
                    cmd.arg(f);
                }
            }
        }

        eprintln!(
            "$ {} {}",
            cmd.get_program().to_string_lossy(),
            cmd.get_args()
                .map(|a| a.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ")
        );

        let start = std::time::Instant::now();

        if let Some(env) = &environment {
            cmd.envs(env);
        }
        run(&mut cmd)?;

        let duration = start.elapsed().as_millis();

        let (output_directory, output_files) = self
            .builder
            .resolve_output_files(&config, environment.as_ref())?;

        if output_files.is_empty() {
            return Err(BuildError::ArtifactNotFound(format!(
                "No .app bundle found in '{}'. \
                 Check your `project`, `scheme`, and `product-name` arguments.",
                output_directory.display()
            )));
        }

        Ok(self
            .builder
            .build_result(config, output_directory, output_files, duration))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn validate_accepts_minimal_valid_config() {
        let mut args = serde_json::Map::new();
        args.insert("project".to_string(), json!("macos/Runner.xcodeproj"));
        args.insert("scheme".to_string(), json!("Runner"));
        let config = BuildConfig::new(args);
        assert!(MacOSXcodeBuilder.validate_arguments(&config).is_ok());
    }

    #[test]
    fn validate_rejects_missing_project() {
        let mut args = serde_json::Map::new();
        args.insert("scheme".to_string(), json!("Runner"));
        let config = BuildConfig::new(args);
        assert!(MacOSXcodeBuilder.validate_arguments(&config).is_err());
    }

    #[test]
    fn validate_rejects_missing_scheme() {
        let mut args = serde_json::Map::new();
        args.insert("project".to_string(), json!("macos/Runner.xcodeproj"));
        let config = BuildConfig::new(args);
        assert!(MacOSXcodeBuilder.validate_arguments(&config).is_err());
    }

    #[test]
    fn matches_target() {
        assert!(MacOSXcodeBuilder.matches(&Platform::MacOS, Some("macos-xcode")));
        assert!(!MacOSXcodeBuilder.matches(&Platform::MacOS, Some("macos")));
        assert!(!MacOSXcodeBuilder.matches(&Platform::MacOS, None));
        assert!(!MacOSXcodeBuilder.matches(&Platform::IOS, Some("macos-xcode")));
    }
}
