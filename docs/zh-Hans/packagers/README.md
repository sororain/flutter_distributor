# 打包器

[English](../../en/packagers/README.md) | 简体中文

本节按平台组织 Fastforge 的打包能力。每个平台页面都会标注当前接入的构建系统、输出格式和环境要求。

## 平台

| 平台                  | 格式                                                          | `package` 当前接入范围                   |
| --------------------- | ------------------------------------------------------------- | ---------------------------------------- |
| [Android](android.md) | [APK](android.md#apk)、[AAB](android.md#aab)                  | Gradle                                   |
| [iOS](ios.md)         | [IPA](ios.md#ipa)                                             | Xcode（package action）                  |
| [macOS](macos.md)     | [DMG](macos.md#dmg)、[PKG](macos.md#pkg)、[ZIP](macos.md#zip) | Xcode（package action）、Flutter Builder |

仓库中还包含 Linux、Windows、Web 和 OpenHarmony 的 packager，但当前 `fastforge package` 尚未把这些路径接入 CLI。本节只为当前可验证的平台和格式提供详细页面。

> [!IMPORTANT]
> Flutter 项目的 `fastforge package` 当前只接通了 macOS 的三种格式；Android 和 iOS 的 `package` 路径只对不含 `pubspec.yaml` 的原生项目有效。构建器的选择规则见[构建器总览](../builders/README.md)。

## 命令选择

- 需要可分发格式：优先使用 `fastforge package`。
- 只需要 Flutter 原始产物：使用 `fastforge build`。
- 需要多任务、输出目录或发布：使用 `fastforge workflow`。

通用流程、参数与钩子见[打包](../packaging.md)。
