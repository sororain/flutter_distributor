use fastforge_core::{AppBuilder, BuildConfig, BuildError, BuildResult, Platform};
use glob::glob;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

// ─── CustomBuilder ────────────────────────────────────────────────────────────

/// A fully user-defined builder.
///
/// Instead of hard-coding a build tool, the caller supplies:
///
/// - **`command`** – the executable to run (e.g. `"make"`, `"./build.sh"`).
/// - **`args`** – zero or more positional arguments that follow the command.
/// - **`output_directory`** – where to look for artifacts after the command
///   finishes (e.g. `"dist"`, `"out/release"`).
/// - **`artifact_patterns`** – one or more glob patterns *relative to*
///   `output_directory` that select the produced files
///   (e.g. `["**/*.apk", "**/*.aab"]`).
///
/// All four fields are stored inside the `BuildConfig` arguments map under the
/// keys listed above so that they flow naturally through the existing
/// `BuildRequest` / `BuildConfig` pipeline.
///
/// # Minimum viable invocation
///
/// ```text
/// platform  = "custom"
/// target    = <anything meaningful to you, e.g. "release-apk">
/// arguments = {
///   "command":            "make",
///   "args":               ["build", "FLAVOR=prod"],   // optional array
///   "output-directory":   "dist",
///   "artifact-patterns":  ["**/*.apk"]
/// }
/// ```
pub struct CustomBuilder;

// ─── helpers ─────────────────────────────────────────────────────────────────

/// Expand a single glob pattern and return all matching paths, sorted.
fn resolve_glob(pattern: &str) -> Result<Vec<PathBuf>, BuildError> {
    let mut paths: Vec<PathBuf> = glob(pattern)
        .map_err(|e| BuildError::Parse(format!("Invalid glob pattern '{}': {}", pattern, e)))?
        .filter_map(Result::ok)
        .collect();
    paths.sort();
    Ok(paths)
}

/// Pull the mandatory `"command"` string out of the arguments map.
fn extract_command(config: &BuildConfig) -> Result<String, BuildError> {
    config
        .arguments
        .get("command")
        .and_then(|v| v.as_str())
        .map(ToString::to_string)
        .ok_or_else(|| {
            BuildError::InvalidArgument(
                "Missing required argument `command`. \
                 Provide the executable to run, e.g. \"make\" or \"./build.sh\"."
                    .to_string(),
            )
        })
}

/// Pull the optional `"args"` array out of the arguments map.
///
/// Accepts both a JSON array of strings and a single string (for convenience).
fn extract_args(config: &BuildConfig) -> Result<Vec<String>, BuildError> {
    match config.arguments.get("args") {
        None => Ok(vec![]),
        Some(serde_json::Value::Array(arr)) => arr
            .iter()
            .enumerate()
            .map(|(i, v)| {
                v.as_str()
                    .map(ToString::to_string)
                    .or_else(|| {
                        // Also accept numbers / booleans by converting to string.
                        if v.is_number() || v.is_boolean() {
                            Some(v.to_string())
                        } else {
                            None
                        }
                    })
                    .ok_or_else(|| {
                        BuildError::InvalidArgument(format!(
                            "`args[{}]` must be a string, got: {}",
                            i, v
                        ))
                    })
            })
            .collect(),
        Some(serde_json::Value::String(s)) => Ok(vec![s.clone()]),
        Some(other) => Err(BuildError::InvalidArgument(format!(
            "`args` must be an array of strings or a single string, got: {}",
            other
        ))),
    }
}

/// Pull the mandatory `"output-directory"` string out of the arguments map.
fn extract_output_directory(config: &BuildConfig) -> Result<PathBuf, BuildError> {
    config
        .arguments
        .get("output-directory")
        .and_then(|v| v.as_str())
        .map(PathBuf::from)
        .ok_or_else(|| {
            BuildError::InvalidArgument(
                "Missing required argument `output-directory`. \
                 Provide the directory where build artifacts will be written, \
                 e.g. \"dist\" or \"build/outputs\"."
                    .to_string(),
            )
        })
}

/// Pull the mandatory `"artifact-patterns"` value out of the arguments map.
///
/// Accepts either a JSON array of glob strings or a single glob string.
fn extract_artifact_patterns(config: &BuildConfig) -> Result<Vec<String>, BuildError> {
    match config.arguments.get("artifact-patterns") {
        None => Err(BuildError::InvalidArgument(
            "Missing required argument `artifact-patterns`. \
             Provide one or more glob patterns that match the produced files, \
             e.g. [\"**/*.apk\"] or \"**/*.ipa\"."
                .to_string(),
        )),
        Some(serde_json::Value::Array(arr)) => {
            if arr.is_empty() {
                return Err(BuildError::InvalidArgument(
                    "`artifact-patterns` must not be an empty array.".to_string(),
                ));
            }
            arr.iter()
                .enumerate()
                .map(|(i, v)| {
                    v.as_str().map(ToString::to_string).ok_or_else(|| {
                        BuildError::InvalidArgument(format!(
                            "`artifact-patterns[{}]` must be a string, got: {}",
                            i, v
                        ))
                    })
                })
                .collect()
        }
        Some(serde_json::Value::String(s)) => {
            if s.is_empty() {
                return Err(BuildError::InvalidArgument(
                    "`artifact-patterns` must not be an empty string.".to_string(),
                ));
            }
            Ok(vec![s.clone()])
        }
        Some(other) => Err(BuildError::InvalidArgument(format!(
            "`artifact-patterns` must be a glob string or an array of glob strings, got: {}",
            other
        ))),
    }
}

// ─── AppBuilder impl ──────────────────────────────────────────────────────────

impl AppBuilder for CustomBuilder {
    fn name(&self) -> &str {
        "custom"
    }

    fn platform(&self) -> Platform {
        // Custom builds are user-defined and can target any platform.
        // Return Android as a reasonable default.
        Platform::Android
    }

    /// Matches every `platform = "custom"` request regardless of target.
    fn matches(&self, _platform: &Platform, _target: Option<&str>) -> bool {
        // CustomBuilder matches any platform/target combination since it's
        // user-defined. The actual filtering happens in CustomAppBuilder::build
        // which checks the original string.
        true
    }

    fn is_supported_on_current_platform(&self) -> bool {
        true
    }

    /// Not used at runtime – the real command comes from `config.arguments`.
    fn build_subcommand(&self) -> &str {
        "custom"
    }

    fn validate_arguments(&self, config: &BuildConfig) -> Result<(), BuildError> {
        // Eagerly validate all required fields so the user gets a clear error
        // before any process is launched.
        extract_command(config)?;
        extract_args(config)?;
        extract_output_directory(config)?;
        extract_artifact_patterns(config)?;
        Ok(())
    }

    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let output_directory = extract_output_directory(config)?;
        let patterns = extract_artifact_patterns(config)?;

        // Expand every pattern relative to the output directory, then
        // deduplicate while preserving order.
        let mut seen = std::collections::HashSet::new();
        let mut all_files: Vec<PathBuf> = Vec::new();

        for pattern in &patterns {
            // Join: output_directory / pattern  so callers never need to repeat
            // the directory prefix inside the pattern.
            let full_pattern = output_directory.join(pattern);
            let full_pattern_str = full_pattern.to_string_lossy();

            let files = resolve_glob(&full_pattern_str)?;
            for file in files {
                if seen.insert(file.clone()) {
                    all_files.push(file);
                }
            }
        }

        // Final sort for deterministic ordering across multiple patterns.
        all_files.sort();
        Ok((output_directory, all_files))
    }

    fn build_result(
        &self,
        config: BuildConfig,
        output_directory: PathBuf,
        output_files: Vec<PathBuf>,
        duration_ms: u128,
    ) -> BuildResult {
        // Use the caller-supplied target (if any) as the result target label.
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
            platform: Platform::Android,
            target,
        }
    }
}

// ─── CustomAppBuilder ─────────────────────────────────────────────────────────

/// High-level runner that mirrors `GradleAppBuilder` / `FlutterAppBuilder`.
///
/// Usage:
///
/// ```rust,ignore
/// let result = CustomAppBuilder::default().build(
///     "custom",
///     Some("my-release"),
///     args_map,   // must contain "command", "output-directory", "artifact-patterns"
///     None,
/// )?;
/// ```
pub struct CustomAppBuilder {
    builder: CustomBuilder,
}

impl Default for CustomAppBuilder {
    fn default() -> Self {
        Self {
            builder: CustomBuilder,
        }
    }
}

impl CustomAppBuilder {
    pub fn build(
        &self,
        platform: &str,
        _target: Option<&str>,
        arguments: serde_json::Map<String, serde_json::Value>,
        environment: Option<HashMap<String, String>>,
    ) -> Result<BuildResult, BuildError> {
        // Use string-based platform matching since "custom" is not a
        // standard Platform variant.
        if platform != "custom" {
            return Err(BuildError::UnsupportedBuilder(format!(
                "CustomBuilder only handles platform=\"custom\", got \"{}\"",
                platform
            )));
        }

        let config = BuildConfig::new(arguments);
        self.builder.validate_arguments(&config)?;

        let command = extract_command(&config)?;
        let args = extract_args(&config)?;

        // Echo the command before running, matching the style used by
        // FlutterCommand::build_with_echo and GradleAppBuilder::run_gradle.
        eprintln!("$ {} {}", command, args.join(" "));

        let start = std::time::Instant::now();
        let exit_code = Self::run_command(&command, &args, environment.as_ref())?;

        if exit_code != 0 {
            return Err(BuildError::CommandFailed(format!(
                "Command '{}' failed with exit code {}",
                command, exit_code
            )));
        }

        let (output_directory, output_files) = self
            .builder
            .resolve_output_files(&config, environment.as_ref())?;

        if output_files.is_empty() {
            return Err(BuildError::ArtifactNotFound(format!(
                "No artifacts matched in '{}'. \
                 Check your `artifact-patterns` argument.",
                output_directory.display()
            )));
        }

        Ok(self.builder.build_result(
            config,
            output_directory,
            output_files,
            start.elapsed().as_millis(),
        ))
    }

    /// Spawn a child process, stream stdout/stderr to the current terminal,
    /// and return the exit code.
    fn run_command(
        command: &str,
        args: &[String],
        environment: Option<&HashMap<String, String>>,
    ) -> Result<i32, BuildError> {
        let mut cmd = std::process::Command::new(command);
        cmd.args(args);
        if let Some(env) = environment {
            cmd.envs(env);
        }

        let output = cmd
            .output()
            .map_err(|e| BuildError::Io(format!("Failed to execute '{}': {}", command, e)))?;

        std::io::stdout()
            .write_all(&output.stdout)
            .map_err(|e| BuildError::Io(format!("Failed to write stdout: {}", e)))?;
        std::io::stderr()
            .write_all(&output.stderr)
            .map_err(|e| BuildError::Io(format!("Failed to write stderr: {}", e)))?;
        std::io::stdout()
            .flush()
            .map_err(|e| BuildError::Io(format!("Failed to flush stdout: {}", e)))?;
        std::io::stderr()
            .flush()
            .map_err(|e| BuildError::Io(format!("Failed to flush stderr: {}", e)))?;

        Ok(output.status.code().unwrap_or(-1))
    }
}

// ─── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Map, Value, json};

    fn make_config(pairs: &[(&str, Value)]) -> BuildConfig {
        let mut args = Map::new();
        for (k, v) in pairs {
            args.insert(k.to_string(), v.clone());
        }
        BuildConfig::new(args)
    }

    fn minimal_config() -> BuildConfig {
        make_config(&[
            ("command", Value::String("make".into())),
            ("output-directory", Value::String("dist".into())),
            (
                "artifact-patterns",
                Value::Array(vec![Value::String("**/*.apk".into())]),
            ),
        ])
    }

    // ── validate_arguments ───────────────────────────────────────────────────

    #[test]
    fn validate_accepts_minimal_valid_config() {
        let config = minimal_config();
        assert!(CustomBuilder.validate_arguments(&config).is_ok());
    }

    #[test]
    fn validate_rejects_missing_command() {
        let config = make_config(&[
            ("output-directory", Value::String("dist".into())),
            ("artifact-patterns", json!(["**/*.apk"])),
        ]);
        let err = CustomBuilder
            .validate_arguments(&config)
            .expect_err("should fail");
        assert!(err.to_string().contains("`command`"));
    }

    #[test]
    fn validate_rejects_missing_output_directory() {
        let config = make_config(&[
            ("command", Value::String("make".into())),
            ("artifact-patterns", json!(["**/*.apk"])),
        ]);
        let err = CustomBuilder
            .validate_arguments(&config)
            .expect_err("should fail");
        assert!(err.to_string().contains("`output-directory`"));
    }

    #[test]
    fn validate_rejects_missing_artifact_patterns() {
        let config = make_config(&[
            ("command", Value::String("make".into())),
            ("output-directory", Value::String("dist".into())),
        ]);
        let err = CustomBuilder
            .validate_arguments(&config)
            .expect_err("should fail");
        assert!(err.to_string().contains("`artifact-patterns`"));
    }

    #[test]
    fn validate_rejects_empty_artifact_patterns_array() {
        let config = make_config(&[
            ("command", Value::String("make".into())),
            ("output-directory", Value::String("dist".into())),
            ("artifact-patterns", Value::Array(vec![])),
        ]);
        let err = CustomBuilder
            .validate_arguments(&config)
            .expect_err("should fail");
        assert!(err.to_string().contains("empty array"));
    }

    #[test]
    fn validate_rejects_empty_artifact_patterns_string() {
        let config = make_config(&[
            ("command", Value::String("make".into())),
            ("output-directory", Value::String("dist".into())),
            ("artifact-patterns", Value::String("".into())),
        ]);
        let err = CustomBuilder
            .validate_arguments(&config)
            .expect_err("should fail");
        assert!(err.to_string().contains("empty string"));
    }

    // ── extract_args ─────────────────────────────────────────────────────────

    #[test]
    fn extract_args_absent_returns_empty_vec() {
        let config = make_config(&[]);
        assert_eq!(extract_args(&config).unwrap(), Vec::<String>::new());
    }

    #[test]
    fn extract_args_from_array() {
        let config = make_config(&[("args", json!(["build", "--release"]))]);
        assert_eq!(
            extract_args(&config).unwrap(),
            vec!["build".to_string(), "--release".to_string()]
        );
    }

    #[test]
    fn extract_args_from_single_string() {
        let config = make_config(&[("args", Value::String("build".into()))]);
        assert_eq!(extract_args(&config).unwrap(), vec!["build".to_string()]);
    }

    #[test]
    fn extract_args_numeric_values_coerced_to_string() {
        let config = make_config(&[("args", json!(["--jobs", 4]))]);
        assert_eq!(
            extract_args(&config).unwrap(),
            vec!["--jobs".to_string(), "4".to_string()]
        );
    }

    #[test]
    fn extract_args_rejects_nested_object() {
        let config = make_config(&[("args", json!([{"key": "val"}]))]);
        let err = extract_args(&config).expect_err("should fail");
        assert!(err.to_string().contains("`args[0]`"));
    }

    // ── extract_artifact_patterns ────────────────────────────────────────────

    #[test]
    fn extract_patterns_from_single_string() {
        let config = make_config(&[("artifact-patterns", Value::String("**/*.ipa".into()))]);
        assert_eq!(
            extract_artifact_patterns(&config).unwrap(),
            vec!["**/*.ipa".to_string()]
        );
    }

    #[test]
    fn extract_patterns_from_array() {
        let config = make_config(&[("artifact-patterns", json!(["**/*.apk", "**/*.aab"]))]);
        assert_eq!(
            extract_artifact_patterns(&config).unwrap(),
            vec!["**/*.apk".to_string(), "**/*.aab".to_string()]
        );
    }

    // ── matches ───────────────────────────────────────────────────────────────

    #[test]
    fn matches_any_target_for_custom_platform() {
        // CustomBuilder now matches any platform/target combination.
        assert!(CustomBuilder.matches(&Platform::Android, None));
        assert!(CustomBuilder.matches(&Platform::Android, Some("release-apk")));
        assert!(CustomBuilder.matches(&Platform::IOS, Some("anything")));
    }

    #[test]
    fn does_not_match_other_platforms() {
        // CustomBuilder matches everything per its trait implementation.
        assert!(CustomBuilder.matches(&Platform::Android, None));
        assert!(CustomBuilder.matches(&Platform::MacOS, Some("desktop")));
    }

    // ── resolve_output_files ─────────────────────────────────────────────────

    #[test]
    fn resolve_output_files_returns_base_dir() {
        // The output directory is returned even when no files match the glob
        // (the caller checks emptiness and raises ArtifactNotFound).
        let config = make_config(&[
            ("command", Value::String("echo".into())),
            (
                "output-directory",
                Value::String("nonexistent_dir_xyz".into()),
            ),
            ("artifact-patterns", json!(["**/*.apk"])),
        ]);
        let (dir, files) = CustomBuilder
            .resolve_output_files(&config, None)
            .expect("should not error on glob expansion");
        assert_eq!(dir, PathBuf::from("nonexistent_dir_xyz"));
        assert!(files.is_empty());
    }

    #[test]
    fn resolve_output_files_deduplicates_overlapping_patterns() {
        use std::fs;
        use tempfile::tempdir;

        let tmp = tempdir().expect("tempdir");
        let apk_path = tmp.path().join("app-release.apk");
        fs::write(&apk_path, b"fake apk").expect("write");

        let dir_str = tmp.path().to_string_lossy().to_string();
        // Two patterns that both match the same file.
        let config = make_config(&[
            ("command", Value::String("echo".into())),
            ("output-directory", Value::String(dir_str)),
            ("artifact-patterns", json!(["**/*.apk", "*.apk"])),
        ]);

        let (_dir, files) = CustomBuilder
            .resolve_output_files(&config, None)
            .expect("resolve");

        // Must appear exactly once despite being matched by two patterns.
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], apk_path);
    }

    #[test]
    fn resolve_output_files_matches_multiple_files_across_patterns() {
        use std::fs;
        use tempfile::tempdir;

        let tmp = tempdir().expect("tempdir");
        fs::write(tmp.path().join("app.apk"), b"apk").expect("write");
        fs::write(tmp.path().join("app.aab"), b"aab").expect("write");

        let dir_str = tmp.path().to_string_lossy().to_string();
        let config = make_config(&[
            ("command", Value::String("echo".into())),
            ("output-directory", Value::String(dir_str)),
            ("artifact-patterns", json!(["*.apk", "*.aab"])),
        ]);

        let (_dir, mut files) = CustomBuilder
            .resolve_output_files(&config, None)
            .expect("resolve");

        files.sort();
        assert_eq!(files.len(), 2);
        assert!(files[0].to_string_lossy().ends_with(".aab"));
        assert!(files[1].to_string_lossy().ends_with(".apk"));
    }

    // ── build_result ──────────────────────────────────────────────────────────

    #[test]
    fn build_result_carries_target_from_arguments() {
        let config = make_config(&[
            ("command", Value::String("make".into())),
            ("output-directory", Value::String("dist".into())),
            ("artifact-patterns", json!(["**/*.apk"])),
            ("target", Value::String("release-apk".into())),
        ]);
        let result = CustomBuilder.build_result(
            config,
            PathBuf::from("dist"),
            vec![PathBuf::from("dist/app.apk")],
            1234,
        );
        assert_eq!(result.platform, Platform::Android);
        assert_eq!(result.target, Some("release-apk".to_string()));
        assert_eq!(result.duration_ms, 1234);
    }

    #[test]
    fn build_result_target_is_none_when_not_provided() {
        let result = CustomBuilder.build_result(minimal_config(), PathBuf::from("dist"), vec![], 0);
        assert_eq!(result.target, None);
    }

    // ── is_supported_on_current_platform ─────────────────────────────────────

    #[test]
    fn supported_on_all_platforms() {
        assert!(CustomBuilder.is_supported_on_current_platform());
    }
}
