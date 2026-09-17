# apk

Build your Flutter app as an Android APK package. The APK format is the traditional Android package format that can be installed directly on Android devices or distributed through app stores and sideloading.

## Requirements

- Flutter SDK installed with Android toolchain configured
- Android device or emulator for testing

## Usage

Run:

```
fastforge package --platform android --targets apk
```

### Build Arguments

Common `build_args` for APK builds:

| Argument | Description |
|----------|-------------|
| `flavor` | Build flavor (e.g., `dev`, `prod`) |
| `target-platform` | Target platforms, e.g. `android-arm,android-arm64` |
| `dart-define` | Dart environment variables |

## Related Links

- [Build and release an Android app](https://docs.flutter.dev/deployment/android)
