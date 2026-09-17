//! Real-toolchain integration tests for the Flutter Builder.
//!
//! These run the actual `flutter` CLI against `fixtures/flutter_app` and
//! assert on real build artifacts. They complement (not replace) the
//! synthetic-`BuildConfig` unit tests already covering argument encoding.

mod support;

use fastforge_app_builder::FlutterAppBuilder;
use fastforge_core::Platform;
use serde_json::{Map, json};
use serial_test::serial;
use support::{WorkingDir, fixture_dir};

#[test]
#[serial]
fn builds_android_apk() {
    let _cwd = WorkingDir::enter(&fixture_dir("flutter_app"));

    let result = FlutterAppBuilder::default()
        .build(&Platform::Android, Some("apk"), Map::new(), None)
        .expect(
            "flutter build apk should succeed against the fixture's debug-signed release config",
        );

    assert!(
        !result.output_files.is_empty(),
        "expected at least one .apk artifact"
    );
    assert!(
        result
            .output_files
            .iter()
            .all(|f| f.extension().and_then(|e| e.to_str()) == Some("apk")),
        "all output files should be .apk: {:?}",
        result.output_files
    );
}

#[test]
#[serial]
fn builds_web() {
    let _cwd = WorkingDir::enter(&fixture_dir("flutter_app"));

    let result = FlutterAppBuilder::default()
        .build(&Platform::Web, None, Map::new(), None)
        .expect("flutter build web should succeed (never requires signing)");

    assert!(
        result.output_directory.join("index.html").exists(),
        "expected build/web/index.html to exist"
    );
}

#[test]
#[serial]
fn builds_macos() {
    let _cwd = WorkingDir::enter(&fixture_dir("flutter_app"));

    let result = FlutterAppBuilder::default()
        .build(&Platform::MacOS, None, Map::new(), None)
        .expect(
            "flutter build macos should succeed; requires a usable local/team \
             codesigning identity in Xcode",
        );

    assert!(!result.output_files.is_empty(), "expected a .app bundle");
}

#[test]
#[serial]
fn builds_ios() {
    let _cwd = WorkingDir::enter(&fixture_dir("flutter_app"));

    let mut arguments = Map::new();
    arguments.insert("export-method".to_string(), json!("development"));

    let result = FlutterAppBuilder::default()
        .build(&Platform::IOS, Some("ipa"), arguments, None)
        .expect(
            "flutter build ipa should succeed; requires a usable development \
             codesigning identity/provisioning profile in Xcode",
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

#[test]
#[ignore = "requires a Linux host; covered by the Phase 5 CI matrix"]
fn builds_linux() {
    let _cwd = WorkingDir::enter(&fixture_dir("flutter_app"));
    let result = FlutterAppBuilder::default()
        .build(&Platform::Linux, None, Map::new(), None)
        .expect("flutter build linux should succeed on a Linux host");
    assert!(!result.output_files.is_empty());
}

#[test]
#[ignore = "requires a Windows host; covered by the Phase 5 CI matrix"]
fn builds_windows() {
    let _cwd = WorkingDir::enter(&fixture_dir("flutter_app"));
    let result = FlutterAppBuilder::default()
        .build(&Platform::Windows, None, Map::new(), None)
        .expect("flutter build windows should succeed on a Windows host");
    assert!(!result.output_files.is_empty());
}
