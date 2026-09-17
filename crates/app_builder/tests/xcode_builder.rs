//! Real-toolchain integration tests for the Xcode Builder (native iOS/macOS
//! projects, no `pubspec.yaml`).
//!
//! These run the actual `xcodebuild` CLI against `fixtures/native_macos` and
//! `fixtures/native_ios` and assert on real build artifacts. The existing
//! `validate_arguments`/matcher unit tests inline in `src/xcode/{ios,macos}.rs`
//! already cover the pure-logic edge cases with synthetic configs; these tests
//! add only the toolchain-dependent, real-artifact coverage.

mod support;

use fastforge_app_builder::{IOSXcodeAppBuilder, MacOSXcodeAppBuilder};
use serde_json::{Map, json};
use serial_test::serial;
use support::{WorkingDir, fixture_dir};

#[test]
#[serial]
fn builds_macos_app() {
    let _cwd = WorkingDir::enter(&fixture_dir("native_macos"));

    let mut arguments = Map::new();
    arguments.insert("project".to_string(), json!("NativeMacOSFixture.xcodeproj"));
    arguments.insert("scheme".to_string(), json!("NativeMacOSFixture"));
    arguments.insert("configuration".to_string(), json!("Release"));
    arguments.insert("derived-data-path".to_string(), json!("build"));

    let result = MacOSXcodeAppBuilder::default()
        .build("macos", None, arguments, None)
        .expect("xcodebuild should succeed for the ad-hoc/unsigned macOS fixture");

    assert!(
        result
            .output_files
            .iter()
            .any(|f| f.extension().and_then(|e| e.to_str()) == Some("app")),
        "expected a .app bundle under build/Build/Products/Release/: {:?}",
        result.output_files
    );
}

#[test]
#[serial]
fn builds_ios_ipa() {
    let _cwd = WorkingDir::enter(&fixture_dir("native_ios"));

    let mut arguments = Map::new();
    arguments.insert("project".to_string(), json!("NativeIOSFixture.xcodeproj"));
    arguments.insert("scheme".to_string(), json!("NativeIOSFixture"));
    arguments.insert("configuration".to_string(), json!("Release"));
    arguments.insert("export-method".to_string(), json!("development"));
    arguments.insert(
        "archive-path".to_string(),
        json!("build/NativeIOSFixture.xcarchive"),
    );
    arguments.insert("export-path".to_string(), json!("build/ipa"));

    let result = IOSXcodeAppBuilder::default()
        .build("ios", None, arguments, None)
        .expect(
            "xcodebuild archive/export should succeed; requires a usable \
             development codesigning identity/provisioning profile in Xcode",
        );

    assert!(
        result
            .output_files
            .iter()
            .any(|f| f.extension().and_then(|e| e.to_str()) == Some("ipa")),
        "expected a .ipa artifact: {:?}",
        result.output_files
    );
}
