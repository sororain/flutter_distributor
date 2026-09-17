# Google Play

[English](../../en/stores/googleplay.md) | 简体中文

`fastforge googleplay` 直接操作 Google Play Developer API，覆盖应用校验、edit、AAB 上传、track 和 catalog。

## 认证

`GOOGLE_PLAY_SERVICE_ACCOUNT_JSON` 可以是完整服务账号 JSON 或文件路径：

```bash
export GOOGLE_PLAY_SERVICE_ACCOUNT_JSON="$PWD/service-account.json"
```

服务账号需要目标应用的 Google Play Developer API 权限。

## 应用

```bash
fastforge googleplay app view com.example.myapp
fastforge googleplay app check com.example.myapp
```

## Edit 工作流

大多数写操作发生在 edit 中：

```bash
fastforge googleplay edit create \
  --package-name com.example.myapp

fastforge googleplay edit commit \
  --package-name com.example.myapp \
  --edit-id <edit-id>
```

不再需要 edit 时可以使用 `edit delete`。

## 上传 AAB

复用现有 edit：

```bash
fastforge googleplay bundle upload dist/app-release.aab \
  --package-name com.example.myapp \
  --edit-id <edit-id>
```

也可以指定 track 并在上传后直接提交：

```bash
fastforge googleplay bundle upload dist/app-release.aab \
  --package-name com.example.myapp \
  --track internal \
  --release-name '1.0.0 (1)' \
  --commit
```

## Track

```bash
fastforge googleplay track list \
  --package-name com.example.myapp \
  --edit-id <edit-id>

fastforge googleplay track view internal \
  --package-name com.example.myapp \
  --edit-id <edit-id>

fastforge googleplay track update internal \
  --package-name com.example.myapp \
  --edit-id <edit-id> \
  --version-code 1 \
  --status completed
```

## Catalog 与原始 API

- Listing、图片和 track 元数据同步见[统一 Catalog](catalog.md)。
- 未封装接口可以通过 `fastforge googleplay api` 调用。
