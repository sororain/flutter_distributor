# Packagers

English | [简体中文](../../zh-Hans/packagers/README.md)

This section organizes Fastforge packaging capabilities by platform. Each platform page describes currently connected build systems, output formats, and environment requirements.

## Platforms

| Platform              | Formats                                                       | Current `package` integration           |
| --------------------- | ------------------------------------------------------------- | --------------------------------------- |
| [Android](android.md) | [APK](android.md#apk), [AAB](android.md#aab)                  | Gradle                                  |
| [iOS](ios.md)         | [IPA](ios.md#ipa)                                             | Xcode (package action)                  |
| [macOS](macos.md)     | [DMG](macos.md#dmg), [PKG](macos.md#pkg), [ZIP](macos.md#zip) | Xcode (package action), Flutter Builder |

The repository also contains packagers for Linux, Windows, Web, and OpenHarmony, but `fastforge package` does not yet expose those paths through the CLI. This section provides detailed pages only for platforms and formats that can currently be verified.

> [!IMPORTANT]
> In Flutter projects, `fastforge package` currently supports only the three macOS formats. The Android and iOS `package` paths apply only to native projects without `pubspec.yaml`. See the [builder overview](../builders/README.md) for builder selection rules.

## Choosing a Command

- To produce a distributable format, prefer `fastforge package`.
- To produce only a raw Flutter artifact, use `fastforge build`.
- For multiple tasks, custom output directories, or publishing, use `fastforge workflow`.

See [Packaging](../packaging.md) for the general process, options, and hooks.
