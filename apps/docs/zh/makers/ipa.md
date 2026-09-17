# ipa

将 Flutter 应用构建为 iOS IPA 包进行分发。IPA 格式用于通过 App Store、TestFlight 或企业部署分发 iOS 应用。它包含编译后的应用包以及 Apple 应用分发系统所需的元数据。

## 环境要求

- 安装了 Xcode 的 macOS 系统
- Apple 开发者账号（用于分发证书和配置文件）
- 已安装 Flutter SDK 并配置了 iOS 工具链

## 使用方法

运行：

```
fastforge package --platform ios --targets ipa --build-export-options-plist ios/exportOptions.plist
```

### 导出选项

iOS 构建需要 `ExportOptions.plist` 文件。以下是一个示例：

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>method</key>
    <string>app-store</string>
</dict>
</plist>
```

常用的导出方法：

| 方法 | 说明 |
|------|------|
| `app-store` | 用于 App Store 分发 |
| `ad-hoc` | 用于在已注册设备上测试 |
| `development` | 用于开发测试 |
| `enterprise` | 用于企业内部分发 |

## 相关链接

- [构建和发布 iOS 应用程序](https://docs.flutter.dev/deployment/ios)
- [App 分发指南](https://developer.apple.com/documentation/xcode/distributing-your-app-for-beta-testing-and-releases)
