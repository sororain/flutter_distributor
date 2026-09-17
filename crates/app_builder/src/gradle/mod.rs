use fastforge_core::{AppBuilder, BuildConfig, BuildError, BuildResult, Platform};
use glob::glob;
use std::collections::HashMap;
use std::path::PathBuf;

// ─── Builder structs ────────────────────────────────────────────────────────

/// Gradle builder for a plain Android project (produces APK).
pub struct GradleAndroidApkBuilder;

/// Gradle builder for a plain Android project (produces AAB).
pub struct GradleAndroidAabBuilder;

/// Gradle builder for a Kotlin Multiplatform project targeting Android (APK).
pub struct GradleKmpAndroidApkBuilder;

/// Gradle builder for a Kotlin Multiplatform project targeting Android (AAB).
pub struct GradleKmpAndroidAabBuilder;

/// Gradle builder for a Kotlin Multiplatform project targeting JVM desktop.
pub struct GradleKmpDesktopBuilder;

/// Gradle builder for a Kotlin Multiplatform project targeting iOS framework.
/// Invokes the Gradle task that assembles the XCFramework consumed by Xcode.
pub struct GradleKmpIosFrameworkBuilder;

// ─── Helpers ────────────────────────────────────────────────────────────────

fn resolve_glob(pattern: &str) -> Result<Vec<PathBuf>, BuildError> {
    let mut output: Vec<PathBuf> = glob(pattern)
        .map_err(|e| BuildError::Parse(format!("Invalid glob pattern '{}': {}", pattern, e)))?
        .filter_map(Result::ok)
        .collect();
    output.sort();
    Ok(output)
}

fn current_platform() -> Option<Platform> {
    Platform::current()
}

/// Returns `"Debug"` / `"Release"` / `"Profile"` with the first letter
/// capitalised – as Gradle task names expect.
fn gradle_build_type(config: &BuildConfig) -> String {
    let raw = config.mode().as_str();
    let mut chars = raw.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => "Release".to_string(),
    }
}

/// Converts a flavor name to the capitalised form used inside Gradle task names.
/// e.g. `"dev"` → `"Dev"`, `"prodFlavor"` → `"ProdFlavor"`.
fn capitalise(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Builds the Gradle task name for assembling / bundling an Android variant.
///
/// | flavor | build_type | assemble=true  | assemble=false          |
/// |--------|------------|----------------|-------------------------|
/// | None   | Release    | assembleRelease | bundleRelease           |
/// | dev    | Release    | assembleDevRelease | bundleDevRelease     |
fn android_gradle_task(config: &BuildConfig, assemble: bool, module: Option<&str>) -> String {
    let verb = if assemble { "assemble" } else { "bundle" };
    let build_type = gradle_build_type(config);

    let variant = if let Some(flavor) = config.flavor() {
        format!("{}{}", capitalise(flavor), build_type)
    } else {
        build_type
    };

    match module {
        Some(m) => format!(":{}:{}{}", m, verb, variant),
        None => format!("{}{}", verb, variant),
    }
}

// ─── GradleAndroidApkBuilder ────────────────────────────────────────────────

impl AppBuilder for GradleAndroidApkBuilder {
    fn name(&self) -> &str {
        "gradle-android"
    }

    fn platform(&self) -> Platform {
        Platform::Android
    }

    fn matches(&self, platform: &Platform, target: Option<&str>) -> bool {
        platform == &Platform::Android && target == Some("apk")
    }

    fn is_supported_on_current_platform(&self) -> bool {
        true
    }

    /// The Gradle wrapper script is the "subcommand" executed by the runner.
    fn build_subcommand(&self) -> &str {
        "assembleRelease"
    }

    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let output_directory = PathBuf::from("app/build/outputs/apk");
        let pattern = if let Some(flavor) = config.flavor() {
            format!(
                "{}/{}/{}/*.apk",
                output_directory.display(),
                flavor.to_lowercase(),
                config.mode().as_str()
            )
        } else {
            format!(
                "{}/{}/*.apk",
                output_directory.display(),
                config.mode().as_str()
            )
        };
        Ok((output_directory, resolve_glob(&pattern)?))
    }

    fn build_result(
        &self,
        config: BuildConfig,
        output_directory: PathBuf,
        output_files: Vec<PathBuf>,
        duration_ms: u128,
    ) -> BuildResult {
        BuildResult {
            config,
            output_directory,
            output_files,
            duration_ms,
            platform: Platform::Android,
            target: Some("apk".to_string()),
        }
    }
}

// ─── GradleAndroidAabBuilder ────────────────────────────────────────────────

impl AppBuilder for GradleAndroidAabBuilder {
    fn name(&self) -> &str {
        "gradle-android"
    }

    fn platform(&self) -> Platform {
        Platform::Android
    }

    fn matches(&self, platform: &Platform, target: Option<&str>) -> bool {
        platform == &Platform::Android && target == Some("aab")
    }

    fn is_supported_on_current_platform(&self) -> bool {
        true
    }

    fn build_subcommand(&self) -> &str {
        "bundleRelease"
    }

    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let output_directory = if let Some(flavor) = config.flavor() {
            PathBuf::from(format!(
                "app/build/outputs/bundle/{}{}",
                flavor.to_lowercase(),
                capitalise(config.mode().as_str())
            ))
        } else {
            PathBuf::from(format!(
                "app/build/outputs/bundle/{}",
                config.mode().as_str()
            ))
        };
        let pattern = format!("{}/*.aab", output_directory.display());
        Ok((output_directory, resolve_glob(&pattern)?))
    }

    fn build_result(
        &self,
        config: BuildConfig,
        output_directory: PathBuf,
        output_files: Vec<PathBuf>,
        duration_ms: u128,
    ) -> BuildResult {
        BuildResult {
            config,
            output_directory,
            output_files,
            duration_ms,
            platform: Platform::Android,
            target: Some("aab".to_string()),
        }
    }
}

// ─── GradleKmpAndroidApkBuilder ─────────────────────────────────────────────

impl AppBuilder for GradleKmpAndroidApkBuilder {
    fn name(&self) -> &str {
        "gradle-kmp"
    }

    fn platform(&self) -> Platform {
        Platform::Android
    }

    fn matches(&self, platform: &Platform, target: Option<&str>) -> bool {
        platform == &Platform::Android && target == Some("android-apk")
    }

    fn is_supported_on_current_platform(&self) -> bool {
        true
    }

    fn build_subcommand(&self) -> &str {
        // Placeholder; the real task is derived from config in `build_subcommand_for`.
        "assembleRelease"
    }

    fn validate_arguments(&self, config: &BuildConfig) -> Result<(), BuildError> {
        // For KMP projects it is best practice to specify which Gradle module to
        // build.  Warn but do not hard-fail so that single-module projects still
        // work without the extra argument.
        if config.arguments.get("module").is_none() {
            eprintln!(
                "[gradle-kmp] Warning: `module` argument not set. \
                 Defaulting to root project task. Pass --module <name> to be explicit."
            );
        }
        Ok(())
    }

    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        // KMP Android module typically outputs to composeApp/build/outputs/apk/
        // or androidApp/build/outputs/apk/ – we search both.
        let module = config
            .arguments
            .get("module")
            .and_then(|v| v.as_str())
            .unwrap_or("composeApp");

        let output_directory = PathBuf::from(format!("{}/build/outputs/apk", module));
        let pattern = if let Some(flavor) = config.flavor() {
            format!(
                "{}/{}/{}/*.apk",
                output_directory.display(),
                flavor.to_lowercase(),
                config.mode().as_str()
            )
        } else {
            format!(
                "{}/{}/*.apk",
                output_directory.display(),
                config.mode().as_str()
            )
        };
        Ok((output_directory, resolve_glob(&pattern)?))
    }

    fn build_result(
        &self,
        config: BuildConfig,
        output_directory: PathBuf,
        output_files: Vec<PathBuf>,
        duration_ms: u128,
    ) -> BuildResult {
        BuildResult {
            config,
            output_directory,
            output_files,
            duration_ms,
            platform: Platform::Android,
            target: Some("android-apk".to_string()),
        }
    }
}

// ─── GradleKmpAndroidAabBuilder ─────────────────────────────────────────────

impl AppBuilder for GradleKmpAndroidAabBuilder {
    fn name(&self) -> &str {
        "gradle-kmp"
    }

    fn platform(&self) -> Platform {
        Platform::Android
    }

    fn matches(&self, platform: &Platform, target: Option<&str>) -> bool {
        platform == &Platform::Android && target == Some("android-aab")
    }

    fn is_supported_on_current_platform(&self) -> bool {
        true
    }

    fn build_subcommand(&self) -> &str {
        "bundleRelease"
    }

    fn validate_arguments(&self, config: &BuildConfig) -> Result<(), BuildError> {
        if config.arguments.get("module").is_none() {
            eprintln!(
                "[gradle-kmp] Warning: `module` argument not set. \
                 Defaulting to root project task."
            );
        }
        Ok(())
    }

    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let module = config
            .arguments
            .get("module")
            .and_then(|v| v.as_str())
            .unwrap_or("composeApp");

        let output_directory = if let Some(flavor) = config.flavor() {
            PathBuf::from(format!(
                "{}/build/outputs/bundle/{}{}",
                module,
                flavor.to_lowercase(),
                capitalise(config.mode().as_str())
            ))
        } else {
            PathBuf::from(format!(
                "{}/build/outputs/bundle/{}",
                module,
                config.mode().as_str()
            ))
        };
        let pattern = format!("{}/*.aab", output_directory.display());
        Ok((output_directory, resolve_glob(&pattern)?))
    }

    fn build_result(
        &self,
        config: BuildConfig,
        output_directory: PathBuf,
        output_files: Vec<PathBuf>,
        duration_ms: u128,
    ) -> BuildResult {
        BuildResult {
            config,
            output_directory,
            output_files,
            duration_ms,
            platform: Platform::Android,
            target: Some("android-aab".to_string()),
        }
    }
}

// ─── GradleKmpDesktopBuilder ─────────────────────────────────────────────────

impl AppBuilder for GradleKmpDesktopBuilder {
    fn name(&self) -> &str {
        "gradle-kmp"
    }

    fn platform(&self) -> Platform {
        // Desktop platform depends on the current host OS.
        Platform::current().unwrap_or(Platform::Linux)
    }

    fn matches(&self, platform: &Platform, target: Option<&str>) -> bool {
        target == Some("desktop")
            && matches!(
                platform,
                Platform::MacOS | Platform::Windows | Platform::Linux
            )
    }

    fn is_supported_on_current_platform(&self) -> bool {
        // Desktop JVM builds can be triggered from any host OS.
        true
    }

    fn build_subcommand(&self) -> &str {
        // Compose Multiplatform desktop packaging task.
        "packageDistributionForCurrentOS"
    }

    fn validate_arguments(&self, config: &BuildConfig) -> Result<(), BuildError> {
        if config.arguments.get("module").is_none() {
            eprintln!(
                "[gradle-kmp] Warning: `module` argument not set. \
                 Defaulting to root project desktop task."
            );
        }
        Ok(())
    }

    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let module = config
            .arguments
            .get("module")
            .and_then(|v| v.as_str())
            .unwrap_or("composeApp");

        // Compose Desktop distributes to <module>/build/compose/binaries/main/
        // with sub-directories per package type (dmg, msi, deb, rpm, …).
        let output_directory = PathBuf::from(format!("{}/build/compose/binaries/main", module));

        let (platform_ext, platform_dir) = match current_platform() {
            Some(Platform::MacOS) => ("dmg", "dmg"),
            Some(Platform::Windows) => ("msi", "msi"),
            Some(Platform::Linux) => ("deb", "deb"),
            _ => {
                return Err(BuildError::UnsupportedPlatform(
                    "Desktop KMP packaging is not supported on this platform".to_string(),
                ));
            }
        };

        let pattern = format!(
            "{}/{}/*.{}",
            output_directory.display(),
            platform_dir,
            platform_ext
        );
        Ok((output_directory, resolve_glob(&pattern)?))
    }

    fn build_result(
        &self,
        config: BuildConfig,
        output_directory: PathBuf,
        output_files: Vec<PathBuf>,
        duration_ms: u128,
    ) -> BuildResult {
        BuildResult {
            config,
            output_directory,
            output_files,
            duration_ms,
            platform: current_platform().unwrap_or(Platform::Linux),
            target: Some("desktop".to_string()),
        }
    }
}

// ─── GradleKmpIosFrameworkBuilder ────────────────────────────────────────────

impl AppBuilder for GradleKmpIosFrameworkBuilder {
    fn name(&self) -> &str {
        "gradle-kmp"
    }

    fn platform(&self) -> Platform {
        Platform::IOS
    }

    fn matches(&self, platform: &Platform, target: Option<&str>) -> bool {
        platform == &Platform::IOS && target == Some("ios-framework")
    }

    fn is_supported_on_current_platform(&self) -> bool {
        // XCFramework assembly requires a macOS host.
        current_platform() == Some(Platform::MacOS)
    }

    fn build_subcommand(&self) -> &str {
        // Kotlin Gradle Plugin task for all three iOS slices as an XCFramework.
        "assembleXCFramework"
    }

    fn validate_arguments(&self, config: &BuildConfig) -> Result<(), BuildError> {
        if config.arguments.get("module").is_none() {
            eprintln!(
                "[gradle-kmp] Warning: `module` argument not set. \
                 Defaulting to shared/. Pass --module <name> if your shared \
                 module has a different name."
            );
        }
        Ok(())
    }

    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let module = config
            .arguments
            .get("module")
            .and_then(|v| v.as_str())
            .unwrap_or("shared");

        let build_type = gradle_build_type(config);

        // KGP outputs XCFramework to:
        //   <module>/build/XCFrameworks/<buildType>/<ModuleName>.xcframework
        let output_directory =
            PathBuf::from(format!("{}/build/XCFrameworks/{}", module, build_type));

        let pattern = format!("{}/*.xcframework", output_directory.display());
        Ok((output_directory, resolve_glob(&pattern)?))
    }

    fn build_result(
        &self,
        config: BuildConfig,
        output_directory: PathBuf,
        output_files: Vec<PathBuf>,
        duration_ms: u128,
    ) -> BuildResult {
        BuildResult {
            config,
            output_directory,
            output_files,
            duration_ms,
            platform: Platform::IOS,
            target: Some("ios-framework".to_string()),
        }
    }
}

// ─── GradleAppBuilder ────────────────────────────────────────────────────────

/// High-level orchestrator that mirrors the pattern used by `FlutterAppBuilder`
/// but drives the Gradle wrapper instead of the Flutter CLI.
pub struct GradleAppBuilder {
    builders: Vec<Box<dyn AppBuilder + Send + Sync>>,
}

impl Default for GradleAppBuilder {
    fn default() -> Self {
        Self {
            builders: vec![
                Box::new(GradleAndroidApkBuilder),
                Box::new(GradleAndroidAabBuilder),
                Box::new(GradleKmpAndroidApkBuilder),
                Box::new(GradleKmpAndroidAabBuilder),
                Box::new(GradleKmpDesktopBuilder),
                Box::new(GradleKmpIosFrameworkBuilder),
            ],
        }
    }
}

impl GradleAppBuilder {
    /// Execute a `./gradlew <task> [args…]` command and stream its output to
    /// stdout/stderr of the current process.
    pub fn run_gradle(
        task: &str,
        extra_args: &[String],
        environment: Option<&HashMap<String, String>>,
    ) -> Result<i32, BuildError> {
        let wrapper = GradleAppBuilder::resolve_wrapper()?;
        eprintln!("$ {} {} {}", wrapper, task, extra_args.join(" "));

        let mut cmd = std::process::Command::new(&wrapper);
        cmd.arg(task).args(extra_args);
        if let Some(env) = environment {
            cmd.envs(env);
        }

        use std::io::Write;
        let output = cmd
            .output()
            .map_err(|e| BuildError::Io(format!("Failed to execute {}: {}", wrapper, e)))?;

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

    /// Locate `gradlew` (Unix) or `gradlew.bat` (Windows) in the current
    /// working directory.  Falls back to a plain `gradle` on `PATH`.
    fn resolve_wrapper() -> Result<String, BuildError> {
        let wrapper = if cfg!(target_os = "windows") {
            "gradlew.bat"
        } else {
            "./gradlew"
        };
        if std::path::Path::new(wrapper).exists() {
            return Ok(wrapper.to_string());
        }
        // If there is no local wrapper, try the system `gradle`.
        Ok("gradle".to_string())
    }

    /// Match a Gradle builder using string-based platform names
    /// (e.g. "gradle-android", "gradle-kmp") since these are not standard
    /// Platform variants. Each builder's `name()` returns the platform prefix.
    ///
    /// Several builders share the same `name()` (e.g. `GradleAndroidApkBuilder`
    /// and `GradleAndroidAabBuilder` are both `"gradle-android"`), so the exact
    /// target must also be checked — delegate to each builder's own `matches`,
    /// called with its own declared `platform()` so only the target actually
    /// disambiguates.
    fn find_builder(
        &self,
        platform: &str,
        target: Option<&str>,
    ) -> Option<&(dyn AppBuilder + Send + Sync)> {
        self.builders
            .iter()
            .find(|b| b.name() == platform && b.matches(&b.platform(), target))
            .map(|b| &**b)
    }

    pub fn build(
        &self,
        platform: &str,
        target: Option<&str>,
        arguments: serde_json::Map<String, serde_json::Value>,
        environment: Option<HashMap<String, String>>,
    ) -> Result<BuildResult, BuildError> {
        let builder = self.find_builder(platform, target).ok_or_else(|| {
            BuildError::UnsupportedBuilder(format!(
                "No Gradle builder found for platform={} target={}",
                platform,
                target.unwrap_or("")
            ))
        })?;

        if !builder.is_supported_on_current_platform() {
            return Err(BuildError::UnsupportedPlatform(format!(
                "{} is not supported on the current platform",
                builder.name()
            )));
        }

        let config = BuildConfig::new(arguments);
        builder.validate_arguments(&config)?;

        // Derive the Gradle task from the module + build type stored in config.
        let module = config
            .arguments
            .get("module")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Use android_gradle_task only for Android targets; other targets use
        // the fixed `build_subcommand`.
        let task = if platform.contains("android")
            || target.map(|t| t.contains("android")).unwrap_or(false)
        {
            let assemble = target.map(|t| t.contains("apk")).unwrap_or(true);
            android_gradle_task(&config, assemble, module.as_deref())
        } else {
            // For desktop / iOS-framework targets, prefix with the module path
            // when specified.
            match module.as_deref() {
                Some(m) => format!(":{}:{}", m, builder.build_subcommand()),
                None => builder.build_subcommand().to_string(),
            }
        };

        // Collect any extra Gradle properties passed as `--gradle-property`
        // arguments (stored as a JSON object in the `gradle-property` key).
        let mut extra_args: Vec<String> = Vec::new();
        if let Some(serde_json::Value::Object(props)) = config.arguments.get("gradle-property") {
            for (k, v) in props {
                extra_args.push(format!("-P{}={}", k, v.as_str().unwrap_or(&v.to_string())));
            }
        }
        // Support passing arbitrary Gradle system-property overrides.
        if let Some(serde_json::Value::Object(props)) = config.arguments.get("system-property") {
            for (k, v) in props {
                extra_args.push(format!("-D{}={}", k, v.as_str().unwrap_or(&v.to_string())));
            }
        }

        let start = std::time::Instant::now();
        let exit_code = GradleAppBuilder::run_gradle(&task, &extra_args, environment.as_ref())?;

        if exit_code != 0 {
            return Err(BuildError::CommandFailed(format!(
                "Gradle task '{}' failed with exit code {}",
                task, exit_code
            )));
        }

        let (output_directory, output_files) =
            builder.resolve_output_files(&config, environment.as_ref())?;

        if output_files.is_empty() {
            return Err(BuildError::ArtifactNotFound(format!(
                "No build artifacts found in '{}'",
                output_directory.display()
            )));
        }

        Ok(builder.build_result(
            config,
            output_directory,
            output_files,
            start.elapsed().as_millis(),
        ))
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Map, Value};

    fn make_config(pairs: &[(&str, Value)]) -> BuildConfig {
        let mut args = Map::new();
        for (k, v) in pairs {
            args.insert(k.to_string(), v.clone());
        }
        BuildConfig::new(args)
    }

    // ── android_gradle_task ──────────────────────────────────────────────────

    #[test]
    fn task_name_assemble_no_flavor() {
        let config = make_config(&[]);
        assert_eq!(android_gradle_task(&config, true, None), "assembleRelease");
    }

    #[test]
    fn task_name_bundle_no_flavor() {
        let config = make_config(&[]);
        assert_eq!(android_gradle_task(&config, false, None), "bundleRelease");
    }

    #[test]
    fn task_name_assemble_with_flavor() {
        let config = make_config(&[("flavor", Value::String("dev".into()))]);
        assert_eq!(
            android_gradle_task(&config, true, None),
            "assembleDevRelease"
        );
    }

    #[test]
    fn task_name_bundle_with_flavor() {
        let config = make_config(&[("flavor", Value::String("prod".into()))]);
        assert_eq!(
            android_gradle_task(&config, false, None),
            "bundleProdRelease"
        );
    }

    #[test]
    fn task_name_with_module() {
        let config = make_config(&[("flavor", Value::String("dev".into()))]);
        assert_eq!(
            android_gradle_task(&config, true, Some("androidApp")),
            ":androidApp:assembleDevRelease"
        );
    }

    #[test]
    fn task_name_profile_mode() {
        let config = make_config(&[("profile", Value::Bool(true))]);
        assert_eq!(android_gradle_task(&config, true, None), "assembleProfile");
    }

    // ── APK output path ──────────────────────────────────────────────────────

    #[test]
    fn android_apk_output_dir_no_flavor() {
        let config = make_config(&[]);
        let (dir, _) = GradleAndroidApkBuilder
            .resolve_output_files(&config, None)
            .expect("resolve");
        assert_eq!(dir.to_string_lossy(), "app/build/outputs/apk");
    }

    #[test]
    fn android_apk_output_pattern_with_flavor() {
        let config = make_config(&[("flavor", Value::String("staging".into()))]);
        let (dir, _) = GradleAndroidApkBuilder
            .resolve_output_files(&config, None)
            .expect("resolve");
        // Directory itself is still the base; the glob narrows it.
        assert_eq!(dir.to_string_lossy(), "app/build/outputs/apk");
    }

    // ── AAB output path ──────────────────────────────────────────────────────

    #[test]
    fn android_aab_output_dir_no_flavor() {
        let config = make_config(&[]);
        let (dir, _) = GradleAndroidAabBuilder
            .resolve_output_files(&config, None)
            .expect("resolve");
        assert_eq!(dir.to_string_lossy(), "app/build/outputs/bundle/release");
    }

    #[test]
    fn android_aab_output_dir_with_flavor() {
        let config = make_config(&[("flavor", Value::String("dev".into()))]);
        let (dir, _) = GradleAndroidAabBuilder
            .resolve_output_files(&config, None)
            .expect("resolve");
        assert_eq!(dir.to_string_lossy(), "app/build/outputs/bundle/devRelease");
    }

    // ── KMP Android APK ──────────────────────────────────────────────────────

    #[test]
    fn kmp_android_apk_default_module() {
        let config = make_config(&[]);
        let (dir, _) = GradleKmpAndroidApkBuilder
            .resolve_output_files(&config, None)
            .expect("resolve");
        assert_eq!(dir.to_string_lossy(), "composeApp/build/outputs/apk");
    }

    #[test]
    fn kmp_android_apk_custom_module() {
        let config = make_config(&[("module", Value::String("androidApp".into()))]);
        let (dir, _) = GradleKmpAndroidApkBuilder
            .resolve_output_files(&config, None)
            .expect("resolve");
        assert_eq!(dir.to_string_lossy(), "androidApp/build/outputs/apk");
    }

    // ── KMP Android AAB ──────────────────────────────────────────────────────

    #[test]
    fn kmp_android_aab_default_module_no_flavor() {
        let config = make_config(&[]);
        let (dir, _) = GradleKmpAndroidAabBuilder
            .resolve_output_files(&config, None)
            .expect("resolve");
        assert_eq!(
            dir.to_string_lossy(),
            "composeApp/build/outputs/bundle/release"
        );
    }

    #[test]
    fn kmp_android_aab_custom_module_with_flavor() {
        let config = make_config(&[
            ("module", Value::String("androidApp".into())),
            ("flavor", Value::String("qa".into())),
        ]);
        let (dir, _) = GradleKmpAndroidAabBuilder
            .resolve_output_files(&config, None)
            .expect("resolve");
        assert_eq!(
            dir.to_string_lossy(),
            "androidApp/build/outputs/bundle/qaRelease"
        );
    }

    // ── KMP iOS Framework ────────────────────────────────────────────────────

    #[test]
    fn kmp_ios_framework_default_module() {
        let config = make_config(&[]);
        let (dir, _) = GradleKmpIosFrameworkBuilder
            .resolve_output_files(&config, None)
            .expect("resolve");
        assert_eq!(dir.to_string_lossy(), "shared/build/XCFrameworks/Release");
    }

    #[test]
    fn kmp_ios_framework_custom_module_profile() {
        let config = make_config(&[
            ("module", Value::String("kmpShared".into())),
            ("profile", Value::Bool(true)),
        ]);
        let (dir, _) = GradleKmpIosFrameworkBuilder
            .resolve_output_files(&config, None)
            .expect("resolve");
        assert_eq!(
            dir.to_string_lossy(),
            "kmpShared/build/XCFrameworks/Profile"
        );
    }

    // ── matcher ──────────────────────────────────────────────────────────────

    #[test]
    fn matchers_are_exclusive() {
        assert!(GradleAndroidApkBuilder.matches(&Platform::Android, Some("apk")));
        assert!(!GradleAndroidApkBuilder.matches(&Platform::Android, Some("aab")));
        assert!(GradleAndroidAabBuilder.matches(&Platform::Android, Some("aab")));
        assert!(!GradleAndroidAabBuilder.matches(&Platform::Android, Some("apk")));
        assert!(GradleKmpAndroidApkBuilder.matches(&Platform::Android, Some("android-apk")));
        assert!(GradleKmpAndroidAabBuilder.matches(&Platform::Android, Some("android-aab")));
        assert!(GradleKmpDesktopBuilder.matches(&Platform::MacOS, Some("desktop")));
        assert!(GradleKmpIosFrameworkBuilder.matches(&Platform::IOS, Some("ios-framework")));
    }

    // ── platform support ─────────────────────────────────────────────────────

    #[test]
    fn android_builders_support_all_platforms() {
        assert!(GradleAndroidApkBuilder.is_supported_on_current_platform());
        assert!(GradleAndroidAabBuilder.is_supported_on_current_platform());
        assert!(GradleKmpAndroidApkBuilder.is_supported_on_current_platform());
        assert!(GradleKmpAndroidAabBuilder.is_supported_on_current_platform());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn ios_framework_supported_on_macos() {
        assert!(GradleKmpIosFrameworkBuilder.is_supported_on_current_platform());
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn ios_framework_not_supported_off_macos() {
        assert!(!GradleKmpIosFrameworkBuilder.is_supported_on_current_platform());
    }
}
