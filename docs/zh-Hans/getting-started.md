# 快速开始

[English](../en/getting-started.md) | 简体中文

本页从安装验证开始，介绍 Fastforge 的打包、发布和产物分析入口。平台细节放在对应的打包器文档中。

## 1. 检查环境

```bash
fastforge --version
fastforge --help
```

请在项目根目录运行命令，并提前安装目标平台所需的 SDK、构建工具和签名工具。

## 2. 选择平台和格式

`package` 会执行平台构建并整理出可分发产物，可用范围取决于项目类型。

原生 Gradle Android 项目可以直接打包 APK 或 AAB：

```bash
fastforge package --platform android --target apk
fastforge package --platform android --target aab
```

Flutter 项目当前只接通了 macOS 格式：

```bash
fastforge package --platform macos --target dmg
```

Flutter 项目的 Android、iOS 等其他平台产物，先用 `fastforge build` 单独生成，见[构建](building.md)。iOS 和 macOS Xcode 项目需要通过工作流传入工程参数，示例见 [Xcode Builder](builders/xcode.md)。执行前可先查看[构建器总览](builders/README.md)和[打包器总览](packagers/README.md)确认覆盖范围。

## 3. 发布已有产物

```bash
fastforge publish \
  --path dist/app.apk \
  --target fir
```

发布参数和凭证见[发布器总览](publishers/README.md)。如果已经有产物，可以跳过打包，直接发布或分析。

## 4. 分析产物

`analyze` 会根据扩展名选择分析器，并输出 JSON：

```bash
fastforge analyze dist/app.apk
```

写入文件：

```bash
fastforge analyze dist/app.apk \
  --output analysis.json
```

当前支持 `.apk`、`.aab`、`.ipa`、`.dmg` 和 macOS `.app` bundle。DMG 与 `.app` 分析只支持 macOS。

## 下一步

- 单独执行构建：[构建](building.md)
- 制作安装包：[打包](packaging.md)
- 上传现有产物：[发布](publishing.md)
- 运行自动化任务：[本地工作流](workflows.md)
- 分析应用产物：[应用包分析](tools/analyze.md)
- 管理商店元数据：[商店管理](stores/README.md)
