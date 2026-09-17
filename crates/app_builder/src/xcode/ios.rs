use fastforge_core::{AppBuilder, BuildConfig, BuildError, BuildResult, Platform};
use glob::glob;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use super::helpers::{extract_optional_str, extract_str, run};

// ─── IOSXcodeBuilder ───────────────────────────────────────────────────────────

/// A builder for native iOS Xcode projects.
///
/// Unlike the Flutter `IOSBuilder`, this runs `xcodebuild` directly:
/// 1. `xcodebuild archive` – builds and archives the `.app` into an `.xcarchive`.
/// 2. `xcodebuild -exportArchive` – exports the `.ipa` from the archive.
///
/// # Minimum viable invocation
///
/// ```text
/// platform = "ios"
/// target   = "ios-xcode"
/// arguments = {
///   "project":            "ios/Runner.xcodeproj",
///   "scheme":             "Runner",
///   "configuration":      "Release",
///   "export-method":      "app-store-connect",   // app-store-connect / ad-hoc / development / enterprise
///   "export-options-plist": "ios/ExportOptions.plist",  // optional, overrides export-method
///   "archive-path":       "ios/build/Runner.xcarchive", // optional
///   "export-path":        "ios/build/ipa",               // optional
///   "derived-data-path":  "ios/build",                   // optional
/// }
/// ```
pub struct IOSXcodeBuilder;

// ─── AppBuilder impl ──────────────────────────────────────────────────────────

impl AppBuilder for IOSXcodeBuilder {
    fn name(&self) -> &str {
        "ios-xcode"
    }

    fn platform(&self) -> Platform {
        Platform::IOS
    }

    fn matches(&self, platform: &Platform, target: Option<&str>) -> bool {
        platform == &Platform::IOS && target == Some("ios-xcode")
    }

    fn is_supported_on_current_platform(&self) -> bool {
        Platform::current() == Some(Platform::MacOS)
    }

    fn build_subcommand(&self) -> &str {
        "ios-xcode"
    }

    fn validate_arguments(&self, config: &BuildConfig) -> Result<(), BuildError> {
        extract_str(config, "project")?;
        extract_str(config, "scheme")?;
        // Either export-method or export-options-plist must be provided
        if !config.arguments.contains_key("export-method")
            && !config.arguments.contains_key("export-options-plist")
        {
            return Err(BuildError::InvalidArgument(
                "Missing `export-method` or `export-options-plist` build argument. \
                 For iOS archive export you must specify how to export the IPA."
                    .to_string(),
            ));
        }
        Ok(())
    }

    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let export_path = extract_optional_str(config, "export-path")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("ios/build/ipa"));

        let mut files: Vec<PathBuf> = glob(&format!("{}/*.ipa", export_path.display()))
            .map_err(|e| BuildError::Parse(format!("Invalid glob pattern: {}", e)))?
            .filter_map(Result::ok)
            .collect();
        files.sort_by(|a, b| {
            let am = a.metadata().and_then(|m| m.modified()).ok();
            let bm = b.metadata().and_then(|m| m.modified()).ok();
            bm.cmp(&am)
        });
        // Return the most recent .ipa
        if let Some(first) = files.first().cloned() {
            Ok((export_path, vec![first]))
        } else {
            Ok((export_path, vec![]))
        }
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
            platform: Platform::IOS,
            target,
        }
    }
}

// ─── IOSXcodeAppBuilder ────────────────────────────────────────────────────────

/// High-level runner for native iOS Xcode projects.
pub struct IOSXcodeAppBuilder {
    builder: IOSXcodeBuilder,
}

impl Default for IOSXcodeAppBuilder {
    fn default() -> Self {
        Self {
            builder: IOSXcodeBuilder,
        }
    }
}

impl IOSXcodeAppBuilder {
    pub fn build(
        &self,
        platform: &str,
        _target: Option<&str>,
        arguments: serde_json::Map<String, serde_json::Value>,
        environment: Option<HashMap<String, String>>,
    ) -> Result<BuildResult, BuildError> {
        if platform != "ios" {
            return Err(BuildError::UnsupportedBuilder(format!(
                "IOSXcodeAppBuilder only handles platform=\"ios\", got \"{}\"",
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
        let archive_path = extract_optional_str(&config, "archive-path")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("ios/build/Runner.xcarchive"));
        let export_path = extract_optional_str(&config, "export-path")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("ios/build/ipa"));
        let export_method = extract_optional_str(&config, "export-method");
        let export_options_plist = extract_optional_str(&config, "export-options-plist");
        let xcconfig = extract_optional_str(&config, "xcconfig-override");

        // Ensure directories exist
        if let Some(parent) = archive_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        std::fs::create_dir_all(&export_path).ok();

        let start = std::time::Instant::now();

        // ── Step 1: Archive ──────────────────────────────────────────────
        let mut archive_cmd = Command::new("xcodebuild");
        archive_cmd.args([
            "archive",
            "-project",
            &project,
            "-scheme",
            &scheme,
            "-configuration",
            &configuration,
            "-archivePath",
            &archive_path.to_string_lossy(),
        ]);

        if let Some(ddp) = derived_data_path {
            archive_cmd.args(["-derivedDataPath", ddp]);
        }

        if let Some(xc) = xcconfig {
            archive_cmd.args(["-xcconfig", xc]);
        }

        // Extra flags
        if let Some(flags) = config
            .arguments
            .get("extra-flags")
            .and_then(|v| v.as_array())
        {
            for flag in flags {
                if let Some(f) = flag.as_str() {
                    archive_cmd.arg(f);
                }
            }
        }

        eprintln!(
            "$ {} {}",
            archive_cmd.get_program().to_string_lossy(),
            archive_cmd
                .get_args()
                .map(|a| a.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ")
        );

        if let Some(env) = &environment {
            archive_cmd.envs(env);
        }
        run(&mut archive_cmd)?;

        // ── Step 2: Export IPA ──────────────────────────────────────────
        let mut export_cmd = Command::new("xcodebuild");
        let mut export_args = vec![
            "-exportArchive".to_string(),
            "-archivePath".to_string(),
            archive_path.to_string_lossy().to_string(),
            "-exportPath".to_string(),
            export_path.to_string_lossy().to_string(),
        ];

        if let Some(plist) = export_options_plist {
            export_args.push("-exportOptionsPlist".to_string());
            export_args.push(plist.to_string());
        } else if let Some(method) = export_method {
            export_args.push("-exportOptionsPlist".to_string());
            // Generate a temporary export options plist
            let tmp_plist = std::env::temp_dir().join("fastforge_export_options.plist");
            let plist_content = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>method</key>
    <string>{}</string>
    <key>signingStyle</key>
    <string>automatic</string>
</dict>
</plist>
"#,
                method
            );
            std::fs::write(&tmp_plist, &plist_content).map_err(|e| {
                BuildError::Io(format!("Failed to write export options plist: {}", e))
            })?;
            export_args.push(tmp_plist.to_string_lossy().to_string());
        }

        let export_cmd_str = format!("xcodebuild {}", export_args.join(" "));
        eprintln!("$ {}", export_cmd_str);

        export_cmd.args(&export_args);
        if let Some(env) = &environment {
            export_cmd.envs(env);
        }
        run(&mut export_cmd)?;

        let duration = start.elapsed().as_millis();

        let (output_directory, output_files) = self
            .builder
            .resolve_output_files(&config, environment.as_ref())?;

        if output_files.is_empty() {
            return Err(BuildError::ArtifactNotFound(format!(
                "No .ipa found in '{}'. Check your archive/export configuration.",
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
    fn validate_accepts_minimal_config_with_export_method() {
        let mut args = serde_json::Map::new();
        args.insert("project".to_string(), json!("ios/Runner.xcodeproj"));
        args.insert("scheme".to_string(), json!("Runner"));
        args.insert("export-method".to_string(), json!("app-store"));
        let config = BuildConfig::new(args);
        assert!(IOSXcodeBuilder.validate_arguments(&config).is_ok());
    }

    #[test]
    fn validate_accepts_export_options_plist_instead_of_method() {
        let mut args = serde_json::Map::new();
        args.insert("project".to_string(), json!("ios/Runner.xcodeproj"));
        args.insert("scheme".to_string(), json!("Runner"));
        args.insert(
            "export-options-plist".to_string(),
            json!("ios/export.plist"),
        );
        let config = BuildConfig::new(args);
        assert!(IOSXcodeBuilder.validate_arguments(&config).is_ok());
    }

    #[test]
    fn validate_rejects_missing_export_config() {
        let mut args = serde_json::Map::new();
        args.insert("project".to_string(), json!("ios/Runner.xcodeproj"));
        args.insert("scheme".to_string(), json!("Runner"));
        let config = BuildConfig::new(args);
        assert!(IOSXcodeBuilder.validate_arguments(&config).is_err());
    }

    #[test]
    fn validate_rejects_missing_project() {
        let mut args = serde_json::Map::new();
        args.insert("scheme".to_string(), json!("Runner"));
        args.insert("export-method".to_string(), json!("ad-hoc"));
        let config = BuildConfig::new(args);
        assert!(IOSXcodeBuilder.validate_arguments(&config).is_err());
    }

    #[test]
    fn matches_target() {
        assert!(IOSXcodeBuilder.matches(&Platform::IOS, Some("ios-xcode")));
        assert!(!IOSXcodeBuilder.matches(&Platform::IOS, Some("ipa")));
        assert!(!IOSXcodeBuilder.matches(&Platform::MacOS, Some("ios-xcode")));
    }
}
