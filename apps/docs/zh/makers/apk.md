# apk

将 Flutter 应用构建为 Android APK 包。APK 是传统的 Android 包格式，可以直接安装到 Android 设备上，或通过应用商店和侧载方式分发。

## 环境要求

- 已安装 Flutter SDK 并配置了 Android 工具链
- Android 设备或模拟器用于测试

## 使用方法

运行：

```
fastforge package --platform android --targets apk
```

### 构建参数

APK 构建常用的 `build_args`：

| 参数 | 说明 |
|------|------|
| `flavor` | 构建变体（例如 `dev`、`prod`） |
| `target-platform` | 目标平台，例如 `android-arm,android-arm64` |
| `dart-define` | Dart 环境变量 |

## 相关链接

- [Build and release an Android app](https://docs.flutter.dev/deployment/android)
