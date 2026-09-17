# fir.im

[English](../../en/publishers/fir.md) | 简体中文

`fir` target 上传 Android APK 或 iOS IPA 到 fir.im。

## 配置

```bash
export FIR_API_TOKEN=fir-api-token
```

发布时必须提供 `bundle_id`：

```bash
fastforge publish --path dist/app.apk --target fir \
  --publish-arg bundle_id=com.example.app
```

## 可选参数

| 参数           | 说明         |
| -------------- | ------------ |
| `app_name`     | 应用显示名称 |
| `version`      | 版本名       |
| `build_number` | 构建号       |

当前只会从 `.apk` 和 `.ipa` 扩展名推断平台。
