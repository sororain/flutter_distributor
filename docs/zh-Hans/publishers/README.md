# 发布目标

[English](../../en/publishers/README.md) | 简体中文

本节按用途拆分当前实际接通的 target。使用 S3 兼容 API 的存储服务合并为一页，减少重复。

## 支持矩阵

| Target             | 用途                       | 文档                        |
| ------------------ | -------------------------- | --------------------------- |
| `s3`、`minio`      | S3 兼容对象存储            | [S3 兼容存储](s3.md)        |
| `qiniu`            | 七牛云对象存储             | [S3 兼容存储](s3.md)        |
| `oss`              | 阿里云 OSS                 | [S3 兼容存储](s3.md)        |
| `cos`              | 腾讯云 COS                 | [S3 兼容存储](s3.md)        |
| `fir`              | fir.im 应用分发            | [fir.im](fir.md)            |
| `firebase`         | Firebase App Distribution  | [Firebase](firebase.md)     |
| `firebase-hosting` | Firebase Hosting           | [Firebase](firebase.md)     |
| `github`           | GitHub Releases            | [GitHub](github.md)         |
| `appstore`         | App Store Connect 构建上传 | [App Store](appstore.md)    |
| `appgallery`       | Huawei AppGallery          | [AppGallery](appgallery.md) |
| `vercel`           | Vercel Production          | [Vercel](vercel.md)         |
| `custom`           | 自定义 shell 命令          | [Custom](custom.md)         |

## 通用用法

```bash
fastforge publish \
  --path <artifact> \
  --target <target> \
  --publish-arg key=value
```

凭证优先使用进程环境变量，不要把密钥写入版本库。

## 当前未接通

- `pgyer` 不是当前有效的通用 target。
- `playstore` 不是通用 target；请使用 `fastforge googleplay`。

需要组合多个步骤时，使用[本地工作流](../workflows.md)。
