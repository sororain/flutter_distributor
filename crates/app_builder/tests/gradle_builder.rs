//! Real-toolchain integration tests for the Gradle Builder (native Android
//! projects, no `pubspec.yaml`).
//!
//! These run the actual `gradle` CLI against `fixtures/native_android` and
//! `fixtures/native_android_flavors` and assert on real build artifacts.
//! Requires `ANDROID_HOME` (or `ANDROID_SDK_ROOT`) to be set in the ambient
//! environment — exactly as a real `fastforge` invocation would.

mod support;

use fastforge_app_builder::GradleAppBuilder;
use serde_json::{Map, Value, json};
use serial_test::serial;
use support::{WorkingDir, fixture_dir};

fn flavor_args(flavor: &str) -> Map<String, Value> {
    let mut args = Map::new();
    args.insert("flavor".to_string(), json!(flavor));
    args
}

#[test]
#[serial]
fn builds_apk_no_flavor() {
    let _cwd = WorkingDir::enter(&fixture_dir("native_android"));

    let result = GradleAppBuilder::default()
        .build("gradle-android", Some("apk"), Map::new(), None)
        .expect("gradle assembleRelease should succeed");

    assert_eq!(
        result.output_directory.to_string_lossy(),
        "app/build/outputs/apk"
    );
    assert!(
        result
            .output_files
            .iter()
            .any(|f| f.extension().and_then(|e| e.to_str()) == Some("apk")),
        "expected an .apk under app/build/outputs/apk/release/: {:?}",
        result.output_files
    );
}

#[test]
#[serial]
fn builds_apk_dev_flavor() {
    let _cwd = WorkingDir::enter(&fixture_dir("native_android_flavors"));

    let result = GradleAppBuilder::default()
        .build("gradle-android", Some("apk"), flavor_args("dev"), None)
        .expect("gradle assembleDevRelease should succeed");

    assert!(
        result
            .output_files
            .iter()
            .any(|f| f.to_string_lossy().contains("dev")
                && f.extension().and_then(|e| e.to_str()) == Some("apk")),
        "expected a dev-flavor .apk: {:?}",
        result.output_files
    );
}

#[test]
#[serial]
fn builds_aab_prod_flavor() {
    let _cwd = WorkingDir::enter(&fixture_dir("native_android_flavors"));

    let result = GradleAppBuilder::default()
        .build("gradle-android", Some("aab"), flavor_args("prod"), None)
        .expect("gradle bundleProdRelease should succeed");

    assert_eq!(
        result.output_directory.to_string_lossy(),
        "app/build/outputs/bundle/prodRelease"
    );
    assert!(
        result
            .output_files
            .iter()
            .any(|f| f.extension().and_then(|e| e.to_str()) == Some("aab")),
        "expected a prod-flavor .aab: {:?}",
        result.output_files
    );
}
