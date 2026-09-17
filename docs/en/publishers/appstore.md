# App Store

English | [简体中文](../../zh-Hans/publishers/appstore.md)

The `appstore` target uses macOS `xcrun altool` to upload an IPA or PKG to App Store Connect.

## Requirements

- macOS and Xcode command-line tools
- A correctly signed `.ipa` or `.pkg`
- One of the following authentication methods

## API Key Authentication

```bash
export APP_STORE_CONNECT_KEY_ID=ABC123DEFG
export APP_STORE_CONNECT_ISSUER_ID=00000000-0000-0000-0000-000000000000
export APP_STORE_CONNECT_KEY_PATH="$PWD/AuthKey_ABC123DEFG.p8"
```

Compatible variables: `APPSTORE_APIKEY` and `APPSTORE_APIISSUER`. When using an API key, the key ID, issuer ID, and key path must all be present.

## Username Authentication

```bash
export APPSTORE_USERNAME=user@example.com
export APPSTORE_PASSWORD=app-specific-password
```

## Upload

```bash
fastforge publish --path dist/MyApp.ipa --target appstore
```

Do not pass sensitive credentials through `--publish-arg` or place them in command history.

## Subsequent Management

Completing an upload does not submit the app for review. Use `fastforge appstore` to query builds, associate versions, and create review submissions; see [App Store Connect](../stores/appstore.md).
