# Flutter Builder

[English](../../en/builders/flutter.md) | 简体中文

Flutter Builder 调用项目环境中的 Flutter CLI，负责执行平台构建、定位原始产物并返回统一的构建结果。顶层 `fastforge build` 当前直接使用该构建器。

## 项目要求

- 在包含 `pubspec.yaml` 的项目根目录执行
- Flutter SDK 已安装，且 `flutter` 位于 `PATH`
- 已准备目标平台的 SDK、签名配置和构建工具

构建器会读取 `pubspec.yaml` 中的 `version`，并把版本名与构建号分别传入 `FLUTTER_BUILD_NAME` 和 `FLUTTER_BUILD_NUMBER`。

## 平台与 target

| 平台      | target       | 输出           |
| --------- | ------------ | -------------- |
| `android` | `apk`        | APK            |
| `android` | `aab`        | AAB            |
| `ios`     | `ipa` 或省略 | IPA            |
| `macos`   | 可省略       | `.app`         |
| `windows` | 可省略       | Windows bundle |
| `linux`   | 可省略       | Linux bundle   |
| `web`     | 可省略       | Web 目录       |
| `ohos`    | `hap`、`app` | HAP 或 APP     |

## 常用命令

```bash
fastforge build --platform android --target apk
fastforge build --platform web
fastforge build --platform macos
```

iOS IPA 必须提供导出配置：

```bash
fastforge build --platform ios --target ipa \
  --build-export-options-plist ios/ExportOptions.plist
```

也可以使用 `--build-export-method`。

## 构建参数

| Fastforge 参数                  | 作用                   |
| ------------------------------- | ---------------------- |
| `--clean`                       | 构建前执行 clean       |
| `--build-target`                | 自定义入口文件         |
| `--build-flavor`                | 指定 flavor            |
| `--build-target-platform`       | 指定目标架构           |
| `--build-export-options-plist`  | iOS 导出配置           |
| `--build-export-method`         | iOS 导出方式           |
| `--build-dart-define KEY=VALUE` | 编译变量；可重复       |
| `--build-obfuscate`             | 开启 obfuscate         |
| `--build-split-debug-info`      | 调试符号输出目录       |
| `--build-tree-shake-icons`      | 开启 icon tree shaking |
| `--build-profile`               | 使用 Profile 模式      |
| `--flutter-build-args`          | 逗号分隔的其他构建参数 |

`--flutter-build-args` 中，无等号条目按布尔开关处理，`key=value` 按键值参数处理。值本身包含逗号时不应使用该入口。

## 产物定位

| 平台            | 默认查找位置                                  |
| --------------- | --------------------------------------------- |
| Android APK     | `build/app/outputs/flutter-apk/`              |
| Android AAB     | `build/app/outputs/bundle/`                   |
| iOS             | `build/ios/ipa/`                              |
| macOS           | `build/macos/Build/Products/`                 |
| Windows         | `build/windows/<arch>/runner/<mode>/`         |
| Linux           | `build/linux/<arch>/<mode>/bundle/`           |
| Web             | `build/web/`                                  |
| OpenHarmony HAP | `ohos/entry/build/<flavor>/outputs/<flavor>/` |
| OpenHarmony APP | `ohos/build/outputs/<flavor>/`                |

构建命令成功但没有在预期目录找到产物时，Fastforge 会返回失败。

## 与打包器的关系

Flutter Builder 可以单独构建上述平台，但当前 `fastforge package` 只接通了它的 macOS DMG、PKG 和 ZIP 路径。Flutter 项目对其他平台执行 `fastforge package` 时，构建会正常完成，随后因对应 packager 未接通而报 `Unsupported package target` 失败；请先使用 `fastforge build` 生成原始产物。

## 宿主限制

- iOS 和 macOS 构建只能在 macOS 上执行。
- Windows 构建只能在 Windows 上执行。
- Linux 构建只能在 Linux 上执行。
- Android、Web 和 OpenHarmony 构建器不设置固定宿主限制，但仍依赖对应平台工具链。
