# hap

Build your Flutter app as an OpenHarmony HAP (HarmonyOS Ability Package) package. The HAP format is the basic deployment unit for OpenHarmony applications, containing the application's resources, assets, and compiled code for installation on OpenHarmony devices.

## Requirements

- Flutter SDK with OpenHarmony (OHOS) toolchain configured
- OpenHarmony SDK and development environment set up
- Release certificate and profile for signing

## Usage

Run:

```shell
fastforge package --platform ohos --targets hap
```

### Build Arguments

Common `build_args` for OpenHarmony HAP builds:

| Argument | Description |
|----------|-------------|
| `dart-define` | Dart environment variables |
| `flavor` | Build flavor (e.g., `dev`, `prod`) |

## Related Links

- [Apply for Release Certificate](https://developer.huawei.com/consumer/en/doc/app/agc-help-add-releasecert-0000001946273961)
- [Apply for Release Profile](https://developer.huawei.com/consumer/en/doc/app/agc-help-add-releaseprofile-0000001914714796)
- [Flutter for OHOS Setup](https://gitcode.com/openharmony-tpc/flutter_flutter/tree/3.22.0-ohos)
