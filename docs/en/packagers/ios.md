# iOS

English | [简体中文](../../zh-Hans/packagers/ios.md)

Fastforge builds and prepares iOS application artifacts through Xcode, producing [IPA](#ipa) files. All iOS operations require macOS and Xcode.

## Current Status

| Build system    | `package` status                         |
| --------------- | ---------------------------------------- |
| Xcode           | IPA connected through the package action |
| Flutter Builder | iOS packager not yet connected           |

Xcode Builder requires `project`, `scheme`, and export configuration, currently supplied through a workflow package action's `build-args`. In a Flutter project, `fastforge package --platform ios` completes the build and then fails because the iOS packager is not connected. Use `fastforge build` to generate the IPA for now.

## IPA

IPA is the archive distribution format for iOS applications. Package an Xcode project through a workflow:

```yaml
- name: Package IPA
  uses: fastforge/package
  with:
    platform: ios
    target: ipa
    output: artifacts/
    build-args: '{"project":"ios/MyApp.xcodeproj","scheme":"MyApp","export-options-plist":"ios/ExportOptions.plist"}'
```

See [Xcode Builder](../builders/xcode.md#ios) for all options.

When building an IPA separately for a Flutter project, provide an export options plist:

```bash
fastforge build \
  --platform ios \
  --target ipa \
  --build-export-options-plist ios/ExportOptions.plist
```

Or use an export method:

```bash
fastforge build \
  --platform ios \
  --target ipa \
  --build-export-method app-store
```

See [Flutter Builder](../builders/flutter.md) for all options.

### Publish to the App Store

```bash
fastforge publish --path dist/MyApp.ipa --target appstore
```

See the [App Store publisher](../publishers/appstore.md) and [App Store Connect](../stores/appstore.md) for credentials, uploads, and review workflows.

Return to the [packager overview](README.md).
