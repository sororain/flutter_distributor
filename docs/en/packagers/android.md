# Android

English | [简体中文](../../zh-Hans/packagers/android.md)

Fastforge builds and prepares Android application artifacts through Gradle, supporting [APK](#apk) and [AAB](#aab).

## Current Status

| Build system    | `package` status                   |
| --------------- | ---------------------------------- |
| Gradle          | APK and AAB supported              |
| Flutter Builder | Android packager not yet connected |

`fastforge package --platform android` applies only to native Gradle projects without `pubspec.yaml`. In a Flutter project, the command completes the build and then fails because the Android packager is not connected. Use `fastforge build` to generate the raw artifact for now.

## Requirements

- Android SDK
- A working Gradle and Android toolchain
- For APK/AAB analysis, configure `ANDROID_HOME` and `aapt2`

## APK

An APK is an Android application package that can be installed directly. For a native Gradle project, run:

```bash
fastforge package --platform android --target apk
```

Fastforge uses Gradle Builder and places the final APK in `dist/`.

Workflow example:

```yaml
- name: Package APK
  uses: fastforge/package
  with:
    platform: android
    target: apk
    output: artifacts/
```

A Flutter project can generate an APK separately:

```bash
fastforge build --platform android --target apk
```

See [Flutter Builder](../builders/flutter.md) for all options.

## AAB

An AAB (Android App Bundle) is used for Google Play distribution. For a native Gradle project, run:

```bash
fastforge package --platform android --target aab
```

A Flutter project can generate an AAB separately:

```bash
fastforge build --platform android --target aab
```

### Upload to Google Play

The general `fastforge publish` command does not currently provide a `playstore` target. Use these commands for uploads and track management:

```bash
fastforge googleplay bundle upload --help
fastforge googleplay track update --help
```

See [Google Play](../stores/googleplay.md) for authentication and command details.

Return to the [packager overview](README.md).
