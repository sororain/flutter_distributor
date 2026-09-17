# Android

[English](../../en/packagers/android.md) | 简体中文

Fastforge 通过 Gradle 构建并整理 Android 应用产物，支持 [APK](#apk) 和 [AAB](#aab) 两种格式。

## 当前状态

| 构建系统        | `package` 状态            |
| --------------- | ------------------------- |
| Gradle          | APK、AAB 已支持           |
| Flutter Builder | 暂未接通 Android packager |

`fastforge package --platform android` 只适用于不含 `pubspec.yaml` 的原生 Gradle 项目。Flutter 项目执行该命令会在构建完成后因 Android packager 未接通而失败，目前请改用 `fastforge build` 生成原始产物。

## 环境要求

- Android SDK
- 可用的 Gradle 和 Android 工具链
- 需要分析 APK/AAB 时，配置 `ANDROID_HOME` 和 `aapt2`

## APK

APK 是 Android 可直接安装的应用包。原生 Gradle 项目直接执行：

```bash
fastforge package --platform android --target apk
```

Fastforge 使用 Gradle Builder，并把最终 APK 整理到 `dist/`。

工作流示例：

```yaml
- name: Package APK
  uses: fastforge/package
  with:
    platform: android
    target: apk
    output: artifacts/
```

Flutter 项目可以单独生成 APK：

```bash
fastforge build --platform android --target apk
```

完整参数见 [Flutter Builder](../builders/flutter.md)。

## AAB

AAB（Android App Bundle）用于 Google Play 分发。原生 Gradle 项目直接执行：

```bash
fastforge package --platform android --target aab
```

Flutter 项目可以单独生成 AAB：

```bash
fastforge build --platform android --target aab
```

### 上传 Google Play

通用 `fastforge publish` 当前没有 `playstore` target。上传和轨道管理请使用：

```bash
fastforge googleplay bundle upload --help
fastforge googleplay track update --help
```

认证和命令说明见 [Google Play](../stores/googleplay.md)。

返回[打包器总览](README.md)。
