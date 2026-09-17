# iOS

[English](../../en/packagers/ios.md) | 简体中文

Fastforge 通过 Xcode 构建并整理 iOS 应用产物，输出格式为 [IPA](#ipa)。所有 iOS 操作都需要 macOS 与 Xcode。

## 当前状态

| 构建系统        | `package` 状态                 |
| --------------- | ------------------------------ |
| Xcode           | IPA 已通过 package action 接入 |
| Flutter Builder | 暂未接通 iOS packager          |

Xcode Builder 需要 `project`、`scheme` 和导出配置，当前通过工作流 package action 的 `build-args` 传入。Flutter 项目执行 `fastforge package --platform ios` 会在构建完成后因 iOS packager 未接通而失败，目前请改用 `fastforge build` 生成 IPA。

## IPA

IPA 是 iOS 应用的归档分发格式。Xcode 项目通过工作流打包：

```yaml
- name: Package IPA
  uses: fastforge/package
  with:
    platform: ios
    target: ipa
    output: artifacts/
    build-args: '{"project":"ios/MyApp.xcodeproj","scheme":"MyApp","export-options-plist":"ios/ExportOptions.plist"}'
```

完整参数见 [Xcode Builder](../builders/xcode.md#ios)。

Flutter 项目单独构建 IPA 时，必须提供 export options plist：

```bash
fastforge build \
  --platform ios \
  --target ipa \
  --build-export-options-plist ios/ExportOptions.plist
```

或使用 export method：

```bash
fastforge build \
  --platform ios \
  --target ipa \
  --build-export-method app-store
```

完整参数见 [Flutter Builder](../builders/flutter.md)。

### 发布到 App Store

```bash
fastforge publish --path dist/MyApp.ipa --target appstore
```

凭证、上传和审核流程见 [App Store 发布器](../publishers/appstore.md)与 [App Store Connect](../stores/appstore.md)。

返回[打包器总览](README.md)。
