# 打包

[English](../en/packaging.md) | 简体中文

`fastforge package` 负责准备项目、执行构建并把产物整理为可分发格式。

```bash
fastforge package --platform android --target apk
```

`--platform` 和 `--target` 均为必填参数，一次处理一个 target。上例适用于原生 Gradle Android 项目；Flutter 项目当前只接通了 macOS 格式（如 `--platform macos --target zip`）。各平台的格式和环境要求见[打包器总览](packagers/README.md)。

## 打包流程

一次打包包含以下阶段：

1. 根据项目文件识别构建系统。
2. 调用对应平台工具完成构建。
3. 运行 pre-package hook。
4. 生成最终分发产物。
5. 运行 post-package hook。

不含 `pubspec.yaml` 的原生项目按平台接入 Gradle（Android）或 Xcode（iOS、macOS）；含 `pubspec.yaml` 的 Flutter 项目一律使用 Flutter Builder 。

## 当前支持范围

| 构建系统        | 适用项目          | 平台    | 格式                | 当前入口            |
| --------------- | ----------------- | ------- | ------------------- | ------------------- |
| Gradle          | 原生 Android      | Android | `apk`、`aab`        | CLI、package action |
| Xcode           | 原生 iOS          | iOS     | `ipa`               | package action      |
| Xcode           | 原生 macOS        | macOS   | `dmg`、`pkg`、`zip` | package action      |
| Flutter Builder | 含 `pubspec.yaml` | macOS   | `dmg`、`pkg`、`zip` | CLI、package action |

> [!NOTE]
> 当前 `fastforge package` 对不同构建系统的覆盖范围并不一致。Flutter 项目当前只能打包 macOS 格式，对 `android`、`ios` 执行 `package` 会在构建完成后因 packager 未接通而失败。路由规则见[构建器总览](builders/README.md)。

## 构建阶段

打包命令会根据项目类型调用 Gradle、Xcode 或 Flutter Builder，再把识别到的产物交给 packager。构建器的选择方式和接入状态见[构建器总览](builders/README.md)。

需要单独检查 Flutter Builder 的结果时，可以运行：

```bash
fastforge build --platform <platform> [--target <target>]
```

平台、target、参数和输出说明见[构建](building.md)。

## 打包参数

### 跳过清理

需要复用已有构建缓存时，可以跳过打包前的清理步骤：

```bash
fastforge package --platform macos --target zip --skip-clean
```

### 自定义 Flutter 入口

```bash
fastforge package --platform macos --target dmg \
  --build-target lib/main_production.dart
```

### 输出目录

直接运行 `package` 时输出目录为 `dist/`。需要自定义目录或产物名时，使用工作流 action 的 `output`、`artifact-name`。

## 生命周期钩子

```bash
fastforge package --platform macos --target zip \
  --hook-pre './scripts/before-package.sh' \
  --hook-post './scripts/after-package.sh'
```

钩子通过系统 shell 执行。Fastforge 会提供以下环境变量：

- `PLATFORM`
- `PACKAGE_FORMAT`
- `BUILD_MODE`
- `OUTPUT_DIRECTORY`
- `BUILD_OUTPUT_DIRECTORY`
- `BUILD_OUTPUT_FILES`（以 `:` 分隔）

任意钩子返回非零退出码时，打包立即失败。

## 自动化

需要组合多个打包目标、发布或其他命令时，使用[本地工作流](workflows.md)。
