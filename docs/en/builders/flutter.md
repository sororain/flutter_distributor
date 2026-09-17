# Flutter Builder

English | [简体中文](../../zh-Hans/builders/flutter.md)

Flutter Builder invokes the Flutter CLI from the project environment, builds for a platform, locates raw artifacts, and returns a normalized build result. The top-level `fastforge build` command currently uses this builder directly.

## Project Requirements

- Run from the root of a project containing `pubspec.yaml`
- Install the Flutter SDK and make `flutter` available in `PATH`
- Prepare the target platform SDK, signing configuration, and build tools

The builder reads `version` from `pubspec.yaml` and passes the version name and build number through `FLUTTER_BUILD_NAME` and `FLUTTER_BUILD_NUMBER` respectively.

## Platforms and Targets

| Platform  | Target           | Output         |
| --------- | ---------------- | -------------- |
| `android` | `apk`            | APK            |
| `android` | `aab`            | AAB            |
| `ios`     | `ipa` or omitted | IPA            |
| `macos`   | May be omitted   | `.app`         |
| `windows` | May be omitted   | Windows bundle |
| `linux`   | May be omitted   | Linux bundle   |
| `web`     | May be omitted   | Web directory  |
| `ohos`    | `hap`, `app`     | HAP or APP     |

## Common Commands

```bash
fastforge build --platform android --target apk
fastforge build --platform web
fastforge build --platform macos
```

An iOS IPA requires export configuration:

```bash
fastforge build --platform ios --target ipa \
  --build-export-options-plist ios/ExportOptions.plist
```

You can also use `--build-export-method`.

## Build Options

| Fastforge option                | Effect                                 |
| ------------------------------- | -------------------------------------- |
| `--clean`                       | Clean before building                  |
| `--build-target`                | Use a custom entry point               |
| `--build-flavor`                | Select a flavor                        |
| `--build-target-platform`       | Select target architectures            |
| `--build-export-options-plist`  | Provide iOS export configuration       |
| `--build-export-method`         | Select an iOS export method            |
| `--build-dart-define KEY=VALUE` | Compile-time variable; repeatable      |
| `--build-obfuscate`             | Enable obfuscation                     |
| `--build-split-debug-info`      | Set the debug-symbol output directory  |
| `--build-tree-shake-icons`      | Enable icon tree shaking               |
| `--build-profile`               | Use Profile mode                       |
| `--flutter-build-args`          | Other build arguments, comma-separated |

Within `--flutter-build-args`, entries without an equals sign are treated as boolean switches, while `key=value` entries become key-value arguments. Do not use this option when a value itself contains a comma.

## Artifact Locations

| Platform        | Default search location                       |
| --------------- | --------------------------------------------- |
| Android APK     | `build/app/outputs/flutter-apk/`              |
| Android AAB     | `build/app/outputs/bundle/`                   |
| iOS             | `build/ios/ipa/`                              |
| macOS           | `build/macos/Build/Products/`                 |
| Windows         | `build/windows/<arch>/runner/<mode>/`         |
| Linux           | `build/linux/<arch>/<mode>/bundle/`           |
| Web             | `build/web/`                                  |
| OpenHarmony HAP | `ohos/entry/build/<flavor>/outputs/<flavor>/` |
| OpenHarmony APP | `ohos/build/outputs/<flavor>/`                |

Fastforge reports a failure if the build command succeeds but no artifact is found in the expected directory.

## Relationship to Packagers

Flutter Builder can build all platforms listed above, but `fastforge package` currently connects only its macOS DMG, PKG, and ZIP paths. When a Flutter project runs `fastforge package` for another platform, the build completes normally and then fails with `Unsupported package target` because the matching packager is not connected. Use `fastforge build` to generate the raw artifact first.

## Host Restrictions

- iOS and macOS builds run only on macOS.
- Windows builds run only on Windows.
- Linux builds run only on Linux.
- Android, Web, and OpenHarmony builders do not enforce a fixed host, but still require the relevant platform toolchains.
