# 构建

[English](../en/building.md) | 简体中文

构建负责调用项目使用的构建系统并定位原始产物。构建结果可以直接用于测试和分析，也可以继续交给[打包器](packagers/README.md)生成可分发格式。

## 构建与打包的区别

| 操作 | 主要职责                     | 常见输出                                     |
| ---- | ---------------------------- | -------------------------------------------- |
| 构建 | 编译项目并定位原始产物       | APK、AAB、IPA、`.app`、桌面 bundle、Web 目录 |
| 打包 | 调用构建器，再整理为分发格式 | DMG、PKG、ZIP，或整理后的 APK、AAB、IPA      |

只需要最终分发文件时，直接使用 `fastforge package`。打包流程会自动调用相应构建器，不必先手动执行 `fastforge build`。

## 当前命令边界

Fastforge 已包含 Gradle、Xcode、Flutter 和 Custom Builder，但它们接入 CLI 的方式不同：

| 构建器               | 当前入口                                        | 状态                     |
| -------------------- | ----------------------------------------------- | ------------------------ |
| Gradle Android       | `fastforge package`、`fastforge/package` action | APK、AAB 已接通          |
| Xcode iOS / macOS    | `fastforge/package` action                      | IPA、macOS `.app` 已接通 |
| Flutter Builder      | `fastforge build`；`package` 仅 macOS 格式      | 已接通多个平台           |
| Gradle Multiplatform | 暂无顶层命令                                    | 仅构建模块可用           |
| Custom Builder       | 暂无顶层命令                                    | 仅构建模块可用           |

> [!IMPORTANT]
> `fastforge build` 当前并不是所有构建器的统一入口。不要使用 `fastforge build` 推断 Gradle、Xcode 或 Custom Builder 是否可用，具体状态见[构建器总览](builders/README.md)。

## 单独执行构建

当前顶层构建命令的格式为：

```bash
fastforge build --platform <platform> [--target <target>]
```

例如：

```bash
fastforge build --platform android --target apk
fastforge build --platform web
```

`--platform` 在执行时必填，部分平台还要求 `--target`。支持的平台、target、构建参数、产物位置和宿主限制统一见 [Flutter Builder](builders/flutter.md)。

## 构建结果

成功后，`fastforge build` 向标准输出写入 JSON，其中包括：

- `config`：构建模式和实际参数
- `platform`：目标平台
- `outputDirectory`：构建输出目录
- `outputFiles`：识别到的产物路径
- `duration`：构建耗时，单位为毫秒

如果构建命令成功但没有在预期目录找到产物，Fastforge 仍会返回失败。这样可以避免后续打包或发布步骤误用空目录。

## 在打包流程中构建

Gradle 和 Xcode Builder 当前主要通过 `package` 使用：

```bash
fastforge package --platform android --target apk
```

Xcode 构建需要 `project`、`scheme` 等参数，当前应通过工作流 action 的 `build-args` 传入。完整示例见 [Xcode Builder](builders/xcode.md)。

## 下一步

- 查看构建器及接入状态：[构建器总览](builders/README.md)
- 生成可分发产物：[打包](packaging.md)
- 分析构建产物：[应用包分析](tools/analyze.md)
- 组合构建、打包和发布：[本地工作流](workflows.md)
