# app

Build your Flutter app as an OpenHarmony APP package. The APP format is the distribution package for OpenHarmony applications, used for publishing to OpenHarmony app stores and installing on OpenHarmony devices.

## Requirements

- Flutter SDK with OpenHarmony (OHOS) toolchain configured
- OpenHarmony SDK and development environment set up
- Release certificate and profile for OpenHarmony distribution

## Usage

Run:

```shell
fastforge package --platform ohos --targets app
```

### Build Arguments

Common `build_args` for OpenHarmony builds:

| Argument | Description |
|----------|-------------|
| `dart-define` | Dart environment variables |
| `flavor` | Build flavor (e.g., `dev`, `prod`) |

## Related Links

- [Apply for Release Certificate](https://developer.huawei.com/consumer/en/doc/app/agc-help-add-releasecert-0000001946273961)
- [Apply for Release Profile](https://developer.huawei.com/consumer/en/doc/app/agc-help-add-releaseprofile-0000001914714796)
- [Flutter for OHOS Setup](https://gitcode.com/openharmony-tpc/flutter_flutter/tree/3.22.0-ohos)
