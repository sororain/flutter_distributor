# ipa

Build your Flutter app as an iOS IPA package for distribution. The IPA format is used to distribute iOS applications through the App Store, TestFlight, or enterprise deployment. It contains the compiled app bundle along with metadata required by Apple's app distribution system.

## Requirements

- macOS with Xcode installed
- Apple Developer account (for distribution certificates and provisioning profiles)
- Flutter SDK with iOS toolchain configured

## Usage

Run:

```
fastforge package --platform ios --targets ipa --build-export-options-plist ios/exportOptions.plist
```

### Export Options

An `ExportOptions.plist` is required for iOS builds. Here is an example:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>method</key>
    <string>app-store</string>
</dict>
</plist>
```

Common export methods:

| Method | Description |
|--------|-------------|
| `app-store` | For App Store distribution |
| `ad-hoc` | For testing on registered devices |
| `development` | For development testing |
| `enterprise` | For in-house enterprise distribution |

## Related Links

- [Build and release an iOS app](https://docs.flutter.dev/deployment/ios)
- [App Distribution Guide](https://developer.apple.com/documentation/xcode/distributing-your-app-for-beta-testing-and-releases)
