# macOS

[English](../../en/packagers/macos.md) | 简体中文

Fastforge 支持把 macOS 应用打包为 [DMG](#dmg)、[PKG](#pkg) 或 [ZIP](#zip)。

## 当前状态

| 构建系统        | `package` 状态                       |
| --------------- | ------------------------------------ |
| Xcode           | DMG、PKG、ZIP 已通过 action 接入     |
| Flutter Builder | DMG、PKG、ZIP 已通过 CLI/action 接入 |

Fastforge 根据项目文件选择 Xcode Builder 或 Flutter Builder，随后把生成的 `.app` 交给对应 packager。Flutter 项目可以直接使用 `fastforge package`；Xcode 项目的工程参数需要通过工作流 `build-args` 传入，见 [Xcode Builder](../builders/xcode.md#macos)。

## 环境要求

- macOS
- Xcode 命令行工具
- 项目自身需要的 SDK 和构建工具

## DMG

DMG 是 macOS 常用的磁盘映像分发格式。当前版本使用内置 DMG maker，不再要求全局安装 `appdmg` Node.js 工具。

Flutter 项目直接执行：

```bash
fastforge package --platform macos --target dmg
```

签名、公证等额外流程可以通过 `package.hooks.post` 执行。

## PKG

PKG 是 macOS 安装器包格式。PKG packager 会读取以下配置文件，文件缺失或无效时打包会失败：

```text
macos/packaging/pkg/make_config.yaml
```

Flutter 项目直接执行：

```bash
fastforge package --platform macos --target pkg
```

需要上传到 App Store 时：

```bash
fastforge publish --path dist/MyApp.pkg --target appstore
```

## ZIP

ZIP 将 macOS `.app` bundle 压缩为便于下载和发布的归档文件。

Flutter 项目直接执行：

```bash
fastforge package --platform macos --target zip
```

打包后可以直接发布，适合 GitHub Releases、S3 兼容存储或自定义下载服务：

```bash
fastforge publish --path dist/MyApp.zip --target github \
  --publish-arg repo=owner/repository \
  --publish-arg release-tag=v1.0.0
```

返回[打包器总览](README.md)。
