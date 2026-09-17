use crate::model::Platform;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Shared metadata about an application that flows through the pipeline.
///
/// This struct is produced by analyzers and consumed by builders, packagers,
/// and publishers, providing a typed contract between pipeline stages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMetadata {
    /// Application name (e.g. "MyApp")
    pub app_name: String,
    /// Binary name (e.g. "my_app")
    pub app_binary_name: Option<String>,
    /// Version string (e.g. "1.0.0+1")
    pub app_version: String,
    /// Build name (e.g. "1.0.0")
    pub build_name: String,
    /// Build number (e.g. "1")
    pub build_number: String,
    /// Package name / bundle identifier (e.g. "com.example.myapp")
    pub package_name: Option<String>,
    /// Target platform.
    pub platform: Option<Platform>,
    /// Flutter version info, if applicable
    pub flutter_version: Option<String>,
    /// Dart version, if applicable
    pub dart_version: Option<String>,
    /// Minimum target OS version
    pub min_target_version: Option<String>,
    /// Arbitrary extra metadata from analysis
    pub extra: HashMap<String, String>,
}

impl AppMetadata {
    pub fn new(app_name: String, app_version: String) -> Self {
        let build_name = app_version
            .split('+')
            .next()
            .unwrap_or(&app_version)
            .to_string();
        let build_number = app_version.split('+').nth(1).unwrap_or("1").to_string();
        Self {
            app_name,
            app_binary_name: None,
            app_version: app_version.clone(),
            build_name,
            build_number,
            package_name: None,
            platform: None,
            flutter_version: None,
            dart_version: None,
            min_target_version: None,
            extra: HashMap::new(),
        }
    }

    /// Parse version from "build_name+build_number" format (e.g. "1.0.0+1")
    pub fn parse_version(version: &str) -> (String, String) {
        let mut split = version.split('+');
        let build_name = split.next().unwrap_or(version).to_string();
        let build_number = split.next().unwrap_or("1").to_string();
        (build_name, build_number)
    }
}
