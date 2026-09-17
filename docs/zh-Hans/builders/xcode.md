# Xcode Builder

[English](../../en/builders/xcode.md) | 简体中文

Xcode Builder 用于 iOS 和 macOS 项目，只能在安装了 Xcode 命令行工具的 macOS 上运行。

## 当前接入范围

| 平台  | 构建结果          | 后续打包               |
| ----- | ----------------- | ---------------------- |
| iOS   | `.xcarchive`、IPA | IPA packager           |
| macOS | `.app`            | DMG、PKG、ZIP packager |

Xcode Builder 已经接入 package 流程，但顶层 `fastforge package` 暂时没有暴露 `project`、`scheme` 等必要参数。当前应通过本地工作流的 `build-args` 使用。

## macOS

```yaml
- name: Package macOS app
  uses: fastforge/package
  with:
    platform: macos
    target: zip
    output: artifacts/
    build-args: '{"project":"macos/MyApp.xcodeproj","scheme":"MyApp","configuration":"Release","product-name":"MyApp"}'
```

构建器执行 `xcodebuild`，然后从构建产品目录中查找 `.app`。

| 参数                | 必填 | 说明                   |
| ------------------- | :--: | ---------------------- |
| `project`           |  是  | `.xcodeproj` 路径      |
| `scheme`            |  是  | Xcode scheme           |
| `configuration`     |  否  | 默认 `Release`         |
| `derived-data-path` |  否  | Derived Data 输出目录  |
| `product-name`      |  否  | 要匹配的 `.app` 名称   |
| `sdk`               |  否  | 传给 `xcodebuild -sdk` |
| `xcconfig-override` |  否  | 额外 xcconfig 文件     |
| `extra-flags`       |  否  | 额外参数数组           |

## iOS

```yaml
- name: Package iOS app
  uses: fastforge/package
  with:
    platform: ios
    target: ipa
    output: artifacts/
    build-args: '{"project":"ios/MyApp.xcodeproj","scheme":"MyApp","configuration":"Release","export-options-plist":"ios/ExportOptions.plist"}'
```

iOS 构建分为两个步骤：

1. 使用 `xcodebuild archive` 生成 `.xcarchive`。
2. 使用 `xcodebuild -exportArchive` 导出 IPA。

| 参数                   |  必填  | 说明                              |
| ---------------------- | :----: | --------------------------------- |
| `project`              |   是   | `.xcodeproj` 路径                 |
| `scheme`               |   是   | Xcode scheme                      |
| `configuration`        |   否   | 默认 `Release`                    |
| `export-options-plist` | 二选一 | ExportOptions plist 路径          |
| `export-method`        | 二选一 | 未提供 plist 时生成临时导出配置   |
| `archive-path`         |   否   | 默认 `ios/build/Runner.xcarchive` |
| `export-path`          |   否   | 默认 `ios/build/ipa`              |
| `derived-data-path`    |   否   | Derived Data 输出目录             |
| `xcconfig-override`    |   否   | 额外 xcconfig 文件                |
| `extra-flags`          |   否   | 额外参数数组                      |

`export-options-plist` 和 `export-method` 至少提供一个。
