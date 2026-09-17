# fir.im

English | [简体中文](../../zh-Hans/publishers/fir.md)

The `fir` target uploads an Android APK or iOS IPA to fir.im.

## Configuration

```bash
export FIR_API_TOKEN=fir-api-token
```

`bundle_id` is required when publishing:

```bash
fastforge publish --path dist/app.apk --target fir \
  --publish-arg bundle_id=com.example.app
```

## Optional Arguments

| Argument       | Description      |
| -------------- | ---------------- |
| `app_name`     | App display name |
| `version`      | Version name     |
| `build_number` | Build number     |

The current implementation infers the platform only from the `.apk` or `.ipa` extension.
