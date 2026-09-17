# aab

Build your Flutter app as an Android App Bundle (AAB) for Google Play Store distribution. The AAB format is the recommended publishing format for Android apps, as it allows Google Play to generate optimized APKs for different device configurations (screen density, CPU architecture, language, etc.).

## Requirements

- Flutter SDK installed with Android toolchain configured
- Java Development Kit (JDK) 11 or newer

## Usage

Run:

```
fastforge package --platform android --targets aab
```

### Build Arguments

Same as APK builds — see the [apk](./apk.md) page for common build arguments, such as `flavor`, `target-platform`, and `dart-define`.

## Related Links

- [Build and release an Android app](https://docs.flutter.dev/deployment/android)
- [About Android App Bundles](https://developer.android.com/platform/technology/app-bundle)
