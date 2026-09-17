# 构建器

[English](../../en/builders/README.md) | 简体中文

构建器负责选择项目构建工具、执行构建命令并定位生成的产物。打包器只消费构建结果，两者的职责不同。

## 当前构建器

| 构建器               | 平台或目标                                            | CLI 接入状态                       | 文档                                      |
| -------------------- | ----------------------------------------------------- | ---------------------------------- | ----------------------------------------- |
| Gradle Android       | Android APK、AAB                                      | 已接入 `package`                   | [Gradle](gradle.md)                       |
| Gradle Multiplatform | Android、桌面、iOS framework                          | 暂未接入顶层 CLI                   | [Gradle](gradle.md#multiplatform-builder) |
| Xcode                | iOS IPA、macOS `.app`                                 | 已接入 `package` action            | [Xcode](xcode.md)                         |
| Flutter              | Android、iOS、macOS、Windows、Linux、Web、OpenHarmony | 已接入 `build`；`package` 仅 macOS | [Flutter Builder](flutter.md)             |
| Custom               | 用户自定义命令和产物规则                              | 暂未接入顶层 CLI                   | [Custom Builder](custom.md)               |

“已实现”与“已接入 CLI”不是同一状态。没有顶层入口的构建器不能直接通过 `fastforge build` 选择。

## 当前路由方式

`fastforge package` 和 `fastforge/package` action 按「项目根目录是否存在 `pubspec.yaml` + `--platform`」选择构建路径：

1. 不存在 `pubspec.yaml` 且 platform 为 `macos` 或 `ios`：选择 Xcode Builder 。
2. 不存在 `pubspec.yaml` 且 platform 为 `android`：选择 Gradle Builder 。
3. 其余情况（包括所有含 `pubspec.yaml` 的 Flutter 项目）：选择 Flutter Builder 。

> [!IMPORTANT]
> Flutter Builder 路径当前只接通了 macOS 的 `dmg`、`pkg`、`zip` packager。Flutter 项目执行 `fastforge package --platform android` 或 `--platform ios` 会先完成构建，然后因对应 packager 未接通而失败；请改用 `fastforge build` 生成原始产物。

当前判断规则较简单。执行命令前应确认工作目录就是项目根目录，避免选择到错误的构建器。

## 如何选择入口

- 原生 Android Gradle 项目：使用 `fastforge package` 或工作流 package action。
- 原生 iOS / macOS Xcode 项目：使用工作流 package action，以 `build-args` 提供工程参数。
- Flutter 项目打包 macOS 格式：使用 `fastforge package`；其他平台先用 `fastforge build` 生成原始产物。
- 需要尚未接入的构建器：当前应继续使用项目自己的构建命令，不要假设 Fastforge CLI 已支持。

构建命令和结果结构见[构建](../building.md)。
