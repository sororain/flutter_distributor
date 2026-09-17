# Gradle Builder

[English](../../en/builders/gradle.md) | 简体中文

Gradle Builder 当前用于 Android 项目的 APK 和 AAB 构建。它优先使用项目根目录的 Gradle Wrapper；找不到 Wrapper 时使用 `PATH` 中的 `gradle`。

## 当前接入范围

| 类型                  | target                       | CLI 状态              |
| --------------------- | ---------------------------- | --------------------- |
| Android               | `apk`                        | 已通过 `package` 接入 |
| Android               | `aab`                        | 已通过 `package` 接入 |
| Multiplatform Android | `android-apk`、`android-aab` | 暂未接入顶层 CLI      |
| Multiplatform Desktop | `desktop`                    | 暂未接入顶层 CLI      |
| Multiplatform iOS     | `ios-framework`              | 暂未接入顶层 CLI      |

## Android 构建

直接打包时，Fastforge 会自动执行 Gradle 构建：

```bash
fastforge package --platform android --target apk
fastforge package --platform android --target aab
```

默认构建 Release 变体。Gradle 任务根据 target、flavor 和 module 生成：

| target | 无 flavor         | flavor 为 `dev`      |
| ------ | ----------------- | -------------------- |
| `apk`  | `assembleRelease` | `assembleDevRelease` |
| `aab`  | `bundleRelease`   | `bundleDevRelease`   |

指定 module 后，任务会增加 module 前缀，例如 `:androidApp:assembleDevRelease`。

## 工作流参数

需要传入 flavor、module 或 Gradle property 时，使用 package action 的 `build-args`：

```yaml
- name: Package Android APK
  uses: fastforge/package
  with:
    platform: android
    target: apk
    output: artifacts/
    build-args: '{"flavor":"dev","module":"app"}'
```

构建器识别以下参数：

| 参数              | 说明                                     |
| ----------------- | ---------------------------------------- |
| `flavor`          | Android product flavor                   |
| `profile`         | 存在该键时使用 Profile，否则使用 Release |
| `module`          | Gradle module 名称                       |
| `gradle-property` | 转换为 `-Pkey=value` 的 JSON object      |
| `system-property` | 转换为 `-Dkey=value` 的 JSON object      |

`build-args` 必须是 JSON object 字符串。

## 产物定位

| target | 默认查找位置                |
| ------ | --------------------------- |
| APK    | `app/build/outputs/apk/`    |
| AAB    | `app/build/outputs/bundle/` |

构建命令成功但没有匹配到对应扩展名时，Fastforge 会把本次构建视为失败。

## Multiplatform Builder

构建模块中还包含 Android APK/AAB、当前宿主桌面分发包和 iOS XCFramework 构建器。它们目前没有接入 `fastforge build`、`fastforge package` 或内置工作流 action，因此本页不提供可直接执行的 Fastforge 命令。
