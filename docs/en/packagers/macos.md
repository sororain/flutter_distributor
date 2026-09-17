# macOS

English | [简体中文](../../zh-Hans/packagers/macos.md)

Fastforge packages macOS applications as [DMG](#dmg), [PKG](#pkg), or [ZIP](#zip).

## Current Status

| Build system    | `package` status                               |
| --------------- | ---------------------------------------------- |
| Xcode           | DMG, PKG, and ZIP connected through the action |
| Flutter Builder | DMG, PKG, and ZIP connected through CLI/action |

Fastforge selects Xcode Builder or Flutter Builder from the project files, then passes the generated `.app` to the matching packager. Flutter projects can use `fastforge package` directly. Xcode projects must pass project arguments through workflow `build-args`; see [Xcode Builder](../builders/xcode.md#macos).

## Requirements

- macOS
- Xcode command-line tools
- SDKs and build tools required by the project

## DMG

DMG is a common disk image distribution format for macOS. The current release uses the built-in DMG maker and no longer requires the globally installed `appdmg` Node.js tool.

For a Flutter project, run:

```bash
fastforge package --platform macos --target dmg
```

Additional steps such as signing and notarization can be run through `package.hooks.post`.

## PKG

PKG is the macOS installer package format. The PKG packager reads this configuration file and fails if it is missing or invalid:

```text
macos/packaging/pkg/make_config.yaml
```

For a Flutter project, run:

```bash
fastforge package --platform macos --target pkg
```

To upload to the App Store:

```bash
fastforge publish --path dist/MyApp.pkg --target appstore
```

## ZIP

ZIP compresses a macOS `.app` bundle into an archive suitable for downloads and publishing.

For a Flutter project, run:

```bash
fastforge package --platform macos --target zip
```

The resulting archive can be published directly to GitHub Releases, S3-compatible storage, or a custom download service:

```bash
fastforge publish --path dist/MyApp.zip --target github \
  --publish-arg repo=owner/repository \
  --publish-arg release-tag=v1.0.0
```

Return to the [packager overview](README.md).
