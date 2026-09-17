# App Store

The appstore target publishes your package artifacts to the [App Store Connect](https://appstoreconnect.apple.com/apps).

## 设置环境变量

需要设置一些环境变量才能正确运行，选择以下其中一种方式即可。

- 用户名和密码

App 专用密码获取：[https://support.apple.com/HT204397](https://support.apple.com/HT204397)

```
export APPSTORE_USERNAME="登录用户名"
export APPSTORE_PASSWORD="App 专用密码"
```

- API 密钥

App Store Connect API: [https://developer.apple.com/documentation/appstoreconnectapi](https://developer.apple.com/documentation/appstoreconnectapi)

```
export APPSTORE_APIKEY="API key"
export APPSTORE_APIISSUER="API issuer"
```

## 用法

运行:

```
fastforge publish \
  --path dist/1.0.0+1/hello_world-1.0.0+1-ios.ipa \
  --targets appstore /
```

### 配置 `distribute_options.yaml`

```yaml
variables:
  APPSTORE_USERNAME: 'xxx'
  APPSTORE_PASSWORD: 'xxx'
  # or
  # APPSTORE_APIKEY: "xxx"
  # APPSTORE_APIISSUER: "xxx"
output: dist/
releases:
  - name: dev
    jobs:
      - name: release-dev-ios
        package:
          platform: ios
          target: ipa
          build_args:
            target: lib/main.dart
            export-options-plist: ios/ExportOptions.plist
        # Publish to appstore
        publish:
          target: appstore
```

> **注意**: 确保你的 `export-options-plist` 中的 `method` 设置为 `app-store`。其他导出方法如 `ad-hoc`、`development` 或 `enterprise` 将导致上传到 App Store Connect 失败。

以下是一个针对 App Store 提交的正确设置的 `ExportOptions.plist` 文件示例：

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>method</key>
    <string>app-store</string>
    <!-- ... other settings ... -->
</dict>
</plist>
```

运行:

```
fastforge release --name dev
```

## 相关链接

- [App 专用密码获取](https://support.apple.com/HT204397)
- [App Store Connect api](https://developer.apple.com/documentation/appstoreconnectapi)
- [altool 指南](https://help.apple.com/asc/appsaltool/)
