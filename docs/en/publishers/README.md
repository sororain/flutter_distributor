# Publishing Targets

English | [简体中文](../../zh-Hans/publishers/README.md)

This section groups the currently connected targets by purpose. Storage services that use S3-compatible APIs share one page to avoid repetition.

## Support Matrix

| Target             | Purpose                        | Documentation                  |
| ------------------ | ------------------------------ | ------------------------------ |
| `s3`, `minio`      | S3-compatible object storage   | [S3-compatible storage](s3.md) |
| `qiniu`            | Qiniu object storage           | [S3-compatible storage](s3.md) |
| `oss`              | Alibaba Cloud OSS              | [S3-compatible storage](s3.md) |
| `cos`              | Tencent Cloud COS              | [S3-compatible storage](s3.md) |
| `fir`              | fir.im app distribution        | [fir.im](fir.md)               |
| `firebase`         | Firebase App Distribution      | [Firebase](firebase.md)        |
| `firebase-hosting` | Firebase Hosting               | [Firebase](firebase.md)        |
| `github`           | GitHub Releases                | [GitHub](github.md)            |
| `appstore`         | App Store Connect build upload | [App Store](appstore.md)       |
| `appgallery`       | Huawei AppGallery              | [AppGallery](appgallery.md)    |
| `vercel`           | Vercel Production              | [Vercel](vercel.md)            |
| `custom`           | Custom shell command           | [Custom](custom.md)            |

## General Usage

```bash
fastforge publish \
  --path <artifact> \
  --target <target> \
  --publish-arg key=value
```

Prefer process environment variables for credentials. Do not commit secrets to the repository.

## Not Currently Connected

- `pgyer` is not a valid general publishing target in the current implementation.
- `playstore` is not a general publishing target; use `fastforge googleplay`.

Use [Local Workflows](../workflows.md) to combine multiple steps.
