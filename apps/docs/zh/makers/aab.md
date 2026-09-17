# aab

将 Flutter 应用构建为 Android App Bundle（AAB）格式，用于 Google Play Store 分发。AAB 是 Android 应用的推荐发布格式，它允许 Google Play 根据不同的设备配置（屏幕密度、CPU 架构、语言等）生成优化的 APK 文件。

## 环境要求

- 已安装 Flutter SDK 并配置了 Android 工具链
- Java Development Kit (JDK) 11 或更高版本

## 使用方法

运行：

```
fastforge package --platform android --targets aab
```

### 构建参数

与 APK 构建相同 — 请参阅 [apk](./apk.md) 页面了解常用的构建参数，如 `flavor`、`target-platform` 和 `dart-define`。

## 相关链接

- [Build and release an Android app](https://docs.flutter.dev/deployment/android)
- [关于 Android App Bundle](https://developer.android.com/platform/technology/app-bundle)
