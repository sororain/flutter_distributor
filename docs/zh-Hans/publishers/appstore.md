# App Store

[English](../../en/publishers/appstore.md) | 简体中文

`appstore` target 使用 macOS `xcrun altool` 上传 IPA 或 PKG 到 App Store Connect。

## 环境要求

- macOS 与 Xcode 命令行工具
- 已正确签名的 `.ipa` 或 `.pkg`
- 以下两种认证方式之一

## API Key 认证

```bash
export APP_STORE_CONNECT_KEY_ID=ABC123DEFG
export APP_STORE_CONNECT_ISSUER_ID=00000000-0000-0000-0000-000000000000
export APP_STORE_CONNECT_KEY_PATH="$PWD/AuthKey_ABC123DEFG.p8"
```

兼容变量：`APPSTORE_APIKEY`、`APPSTORE_APIISSUER`。使用 API Key 时，key id、issuer id 和 key path 必须同时存在。

## 用户名认证

```bash
export APPSTORE_USERNAME=user@example.com
export APPSTORE_PASSWORD=app-specific-password
```

## 上传

```bash
fastforge publish --path dist/MyApp.ipa --target appstore
```

敏感凭证不要通过 `--publish-arg` 传递或写入命令历史。

## 后续管理

上传完成不等于提交审核。查询构建、关联版本和创建审核 submission 请使用 `fastforge appstore`，见 [App Store Connect](../stores/appstore.md)。
