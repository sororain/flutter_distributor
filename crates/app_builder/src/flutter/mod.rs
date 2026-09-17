use fastforge_core::{AppBuilder, BuildConfig, BuildError, BuildResult, Platform};
use glob::glob;
use std::collections::HashMap;
use std::path::PathBuf;

pub mod command;
pub mod flutter_version;
pub mod pubspec_info;

pub use flutter_version::FlutterVersion;
pub use pubspec_info::PubspecInfo;

pub struct AndroidApkBuilder;
pub struct AndroidAabBuilder;
pub struct IOSBuilder;
pub struct MacOSBuilder;
pub struct WindowsBuilder;
pub struct LinuxBuilder;
pub struct WebBuilder;
pub struct OhosHapBuilder;
pub struct OhosAppBuilder;

fn sentence_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

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

/// Parses `PRODUCT_NAME` from a macOS `AppInfo.xcconfig` file.
fn read_product_name_from_xcconfig(path: &str) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    for line in content.lines() {
        let line = line.trim();
        // Skip comments and empty lines
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some(value) = line.strip_prefix("PRODUCT_NAME")
            && let Some(equals) = value.strip_prefix('=')
        {
            let name = equals.trim();
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
    }
    None
}

impl AppBuilder for AndroidApkBuilder {
    fn name(&self) -> &str {
        "android"
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
    fn build_subcommand(&self) -> &str {
        "apk"
    }
    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let output_directory = PathBuf::from("build/app/outputs/flutter-apk");
        let pattern = if let Some(flavor) = config.flavor() {
            format!(
                "{}/**/*-{}-{}.apk",
                output_directory.display(),
                flavor.to_lowercase(),
                config.mode().as_str()
            )
        } else {
            format!(
                "{}/**/*-{}.apk",
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

impl AppBuilder for AndroidAabBuilder {
    fn name(&self) -> &str {
        "android"
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
        "appbundle"
    }
    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let mut build_mode = config.mode().as_str().to_string();
        let output_directory = if let Some(flavor) = config.flavor() {
            build_mode = sentence_case(&build_mode);
            PathBuf::from(format!("build/app/outputs/bundle/{}{}", flavor, build_mode))
        } else {
            PathBuf::from(format!("build/app/outputs/bundle/{}", build_mode))
        };
        let pattern = if let Some(flavor) = config.flavor() {
            format!(
                "{}/**/*-{}-{}.aab",
                output_directory.display(),
                flavor,
                config.mode().as_str()
            )
        } else {
            format!(
                "{}/**/*-{}.aab",
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
            target: Some("aab".to_string()),
        }
    }
}

impl AppBuilder for IOSBuilder {
    fn name(&self) -> &str {
        "ios"
    }
    fn platform(&self) -> Platform {
        Platform::IOS
    }
    fn matches(&self, platform: &Platform, target: Option<&str>) -> bool {
        platform == &Platform::IOS && target.is_none_or(|t| t == "ipa")
    }
    fn is_supported_on_current_platform(&self) -> bool {
        current_platform() == Some(Platform::MacOS)
    }
    fn build_subcommand(&self) -> &str {
        "ipa"
    }
    fn validate_arguments(&self, config: &BuildConfig) -> Result<(), BuildError> {
        if !config.arguments.contains_key("export-options-plist")
            && !config.arguments.contains_key("export-method")
        {
            return Err(BuildError::InvalidArgument(
                "Missing `export-options-plist` or `export-method` build argument.".to_string(),
            ));
        }
        Ok(())
    }
    fn resolve_output_files(
        &self,
        _config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let output_directory = PathBuf::from("build/ios/ipa");
        let mut files = resolve_glob(&format!("{}/**/*.ipa", output_directory.display()))?;
        files.sort_by(|a, b| {
            let am = a.metadata().and_then(|m| m.modified()).ok();
            let bm = b.metadata().and_then(|m| m.modified()).ok();
            bm.cmp(&am)
        });
        if let Some(first) = files.first().cloned() {
            Ok((output_directory, vec![first]))
        } else {
            Ok((output_directory, vec![]))
        }
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
            target: Some("ipa".to_string()),
        }
    }
}

impl AppBuilder for MacOSBuilder {
    fn name(&self) -> &str {
        "macos"
    }
    fn platform(&self) -> Platform {
        Platform::MacOS
    }
    fn matches(&self, platform: &Platform, _target: Option<&str>) -> bool {
        platform == &Platform::MacOS
    }
    fn is_supported_on_current_platform(&self) -> bool {
        current_platform() == Some(Platform::MacOS)
    }
    fn build_subcommand(&self) -> &str {
        "macos"
    }
    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let build_mode = sentence_case(config.mode().as_str());
        let output_directory = if let Some(flavor) = config.flavor() {
            PathBuf::from(format!(
                "build/macos/Build/Products/{}-{}",
                build_mode, flavor
            ))
        } else {
            PathBuf::from(format!("build/macos/Build/Products/{}", build_mode))
        };

        // Read PRODUCT_NAME from AppInfo.xcconfig to target the exact .app bundle.
        // Falls back to a wildcard if the file is missing or unparseable.
        let product_name = read_product_name_from_xcconfig("macos/Runner/Configs/AppInfo.xcconfig")
            .unwrap_or_else(|| "*".to_string());

        let files = resolve_glob(&format!(
            "{}/{}.app",
            output_directory.display(),
            product_name
        ))?;
        Ok((output_directory, files))
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
            platform: Platform::MacOS,
            target: None,
        }
    }
}

impl AppBuilder for WindowsBuilder {
    fn name(&self) -> &str {
        "windows"
    }
    fn platform(&self) -> Platform {
        Platform::Windows
    }
    fn matches(&self, platform: &Platform, _target: Option<&str>) -> bool {
        platform == &Platform::Windows
    }
    fn is_supported_on_current_platform(&self) -> bool {
        current_platform() == Some(Platform::Windows)
    }
    fn build_subcommand(&self) -> &str {
        "windows"
    }
    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let build_mode = sentence_case(config.mode().as_str());
        let arch = if let Some(env) = environment {
            let upper = env
                .get("PROCESSOR_ARCHITECTURE")
                .map(|v| v.to_ascii_uppercase())
                .unwrap_or_default();
            if upper == "ARM64" { "arm64" } else { "x64" }
        } else {
            "x64"
        };

        let output_directory =
            PathBuf::from(format!("build/windows/{}/runner/{}", arch, build_mode));
        let files = resolve_glob(&format!("{}/**/*", output_directory.display()))?;
        Ok((output_directory, files))
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
            platform: Platform::Windows,
            target: None,
        }
    }
}

impl AppBuilder for LinuxBuilder {
    fn name(&self) -> &str {
        "linux"
    }
    fn platform(&self) -> Platform {
        Platform::Linux
    }
    fn matches(&self, platform: &Platform, _target: Option<&str>) -> bool {
        platform == &Platform::Linux
    }
    fn is_supported_on_current_platform(&self) -> bool {
        current_platform() == Some(Platform::Linux)
    }
    fn build_subcommand(&self) -> &str {
        "linux"
    }
    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let arch = if std::env::consts::ARCH == "aarch64" {
            "arm64"
        } else {
            "x64"
        };
        let output_directory = PathBuf::from(format!(
            "build/linux/{}/{}/bundle",
            arch,
            config.mode().as_str()
        ));
        let files = resolve_glob(&format!("{}/**/*", output_directory.display()))?;
        Ok((output_directory, files))
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
            platform: Platform::Linux,
            target: None,
        }
    }
}

impl AppBuilder for WebBuilder {
    fn name(&self) -> &str {
        "web"
    }
    fn platform(&self) -> Platform {
        Platform::Web
    }
    fn matches(&self, platform: &Platform, _target: Option<&str>) -> bool {
        platform == &Platform::Web
    }
    fn is_supported_on_current_platform(&self) -> bool {
        true
    }
    fn build_subcommand(&self) -> &str {
        "web"
    }
    fn resolve_output_files(
        &self,
        _config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let output_directory = PathBuf::from("build/web");
        let files = resolve_glob(&format!("{}/**/*", output_directory.display()))?;
        Ok((output_directory, files))
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
            platform: Platform::Web,
            target: None,
        }
    }
}

impl AppBuilder for OhosHapBuilder {
    fn name(&self) -> &str {
        "ohos"
    }
    fn platform(&self) -> Platform {
        Platform::Ohos
    }
    fn matches(&self, platform: &Platform, target: Option<&str>) -> bool {
        platform == &Platform::Ohos && target == Some("hap")
    }
    fn is_supported_on_current_platform(&self) -> bool {
        true
    }
    fn build_subcommand(&self) -> &str {
        "hap"
    }
    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let flavor = config.flavor().unwrap_or("default");
        let output_directory = PathBuf::from(format!("ohos/entry/build/{0}/outputs/{0}", flavor));
        let pattern = format!("{}/**/*-{}-signed.hap", output_directory.display(), flavor);
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
            platform: Platform::Ohos,
            target: Some("hap".to_string()),
        }
    }
}

impl AppBuilder for OhosAppBuilder {
    fn name(&self) -> &str {
        "ohos"
    }
    fn platform(&self) -> Platform {
        Platform::Ohos
    }
    fn matches(&self, platform: &Platform, target: Option<&str>) -> bool {
        platform == &Platform::Ohos && target == Some("app")
    }
    fn is_supported_on_current_platform(&self) -> bool {
        true
    }
    fn build_subcommand(&self) -> &str {
        "app"
    }
    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        _environment: Option<&HashMap<String, String>>,
    ) -> Result<(PathBuf, Vec<PathBuf>), BuildError> {
        let flavor = config.flavor().unwrap_or("default");
        let output_directory = PathBuf::from(format!("ohos/build/outputs/{}", flavor));
        let pattern = format!("{}/**/*-{}-signed.app", output_directory.display(), flavor);
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
            platform: Platform::Ohos,
            target: Some("app".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Map, Value};

    #[test]
    fn android_apk_path_resolution() {
        let mut args = Map::new();
        args.insert("flavor".to_string(), Value::String("dev".to_string()));
        let config = BuildConfig::new(args);
        let (dir, _) = AndroidApkBuilder
            .resolve_output_files(&config, None)
            .expect("resolve should work");
        assert_eq!(dir.to_string_lossy(), "build/app/outputs/flutter-apk");
    }

    #[test]
    fn ios_argument_validation() {
        let config = BuildConfig::new(Map::new());
        let err = IOSBuilder
            .validate_arguments(&config)
            .expect_err("must fail");
        assert!(
            err.to_string()
                .contains("Missing `export-options-plist` or `export-method`")
        );
    }

    #[test]
    fn windows_path_for_new_flutter_version() {
        let config = BuildConfig::new(Map::new());
        let (path, _) = WindowsBuilder
            .resolve_output_files(&config, None)
            .expect("resolve path");
        assert_eq!(path.to_string_lossy(), "build/windows/x64/runner/Release");
    }

    #[test]
    fn ohos_default_flavor() {
        let config = BuildConfig::new(Map::new());
        let (path, _) = OhosHapBuilder
            .resolve_output_files(&config, None)
            .expect("resolve path");
        assert_eq!(
            path.to_string_lossy(),
            "ohos/entry/build/default/outputs/default"
        );
    }

    #[test]
    fn macos_profile_mode_path() {
        let mut args = Map::new();
        args.insert("profile".to_string(), Value::Bool(true));
        let config = BuildConfig::new(args);
        let (path, _) = MacOSBuilder
            .resolve_output_files(&config, None)
            .expect("resolve path");
        assert_eq!(path.to_string_lossy(), "build/macos/Build/Products/Profile");
    }

    #[test]
    fn android_aab_flavor_path() {
        let mut args = Map::new();
        args.insert("flavor".to_string(), Value::String("dev".to_string()));
        let config = BuildConfig::new(args);
        let (path, _) = AndroidAabBuilder
            .resolve_output_files(&config, None)
            .expect("resolve path");
        assert_eq!(
            path.to_string_lossy(),
            "build/app/outputs/bundle/devRelease"
        );
    }
}
