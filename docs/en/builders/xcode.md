# Xcode Builder

English | [简体中文](../../zh-Hans/builders/xcode.md)

Xcode Builder supports iOS and macOS projects and runs only on macOS with the Xcode command-line tools installed.

## Current Integration

| Platform | Build result      | Subsequent packaging        |
| -------- | ----------------- | --------------------------- |
| iOS      | `.xcarchive`, IPA | IPA packager                |
| macOS    | `.app`            | DMG, PKG, and ZIP packagers |

Xcode Builder is connected to the package process, but the top-level `fastforge package` command does not yet expose required arguments such as `project` and `scheme`. Use it through a local workflow's `build-args` for now.

## macOS

```yaml
- name: Package macOS app
  uses: fastforge/package
  with:
    platform: macos
    target: zip
    output: artifacts/
    build-args: '{"project":"macos/MyApp.xcodeproj","scheme":"MyApp","configuration":"Release","product-name":"MyApp"}'
```

The builder runs `xcodebuild`, then searches the build products directory for an `.app`.

| Argument            | Required | Description                   |
| ------------------- | :------: | ----------------------------- |
| `project`           |   Yes    | Path to `.xcodeproj`          |
| `scheme`            |   Yes    | Xcode scheme                  |
| `configuration`     |    No    | Defaults to `Release`         |
| `derived-data-path` |    No    | Derived Data output directory |
| `product-name`      |    No    | `.app` name to match          |
| `sdk`               |    No    | Passed to `xcodebuild -sdk`   |
| `xcconfig-override` |    No    | Additional xcconfig file      |
| `extra-flags`       |    No    | Array of additional arguments |

## iOS

```yaml
- name: Package iOS app
  uses: fastforge/package
  with:
    platform: ios
    target: ipa
    output: artifacts/
    build-args: '{"project":"ios/MyApp.xcodeproj","scheme":"MyApp","configuration":"Release","export-options-plist":"ios/ExportOptions.plist"}'
```

An iOS build has two stages:

1. Run `xcodebuild archive` to produce an `.xcarchive`.
2. Run `xcodebuild -exportArchive` to export the IPA.

| Argument               |   Required   | Description                                                       |
| ---------------------- | :----------: | ----------------------------------------------------------------- |
| `project`              |     Yes      | Path to `.xcodeproj`                                              |
| `scheme`               |     Yes      | Xcode scheme                                                      |
| `configuration`        |      No      | Defaults to `Release`                                             |
| `export-options-plist` | One required | Path to an ExportOptions plist                                    |
| `export-method`        | One required | Generate temporary export configuration when no plist is supplied |
| `archive-path`         |      No      | Defaults to `ios/build/Runner.xcarchive`                          |
| `export-path`          |      No      | Defaults to `ios/build/ipa`                                       |
| `derived-data-path`    |      No      | Derived Data output directory                                     |
| `xcconfig-override`    |      No      | Additional xcconfig file                                          |
| `extra-flags`          |      No      | Array of additional arguments                                     |

Provide at least one of `export-options-plist` and `export-method`.
