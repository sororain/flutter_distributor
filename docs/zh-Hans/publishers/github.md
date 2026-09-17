# GitHub Releases

[English](../../en/publishers/github.md) | 简体中文

`github` target 把文件上传到 GitHub Release；release 不存在时会创建。

## 认证

```bash
export GITHUB_TOKEN=github-token
```

Token 需要目标仓库 Release 的读写权限。

## 发布

```bash
fastforge publish --path dist/app.zip --target github \
  --publish-arg repo=owner/repository \
  --publish-arg release-tag=v1.0.0
```

CI 中可以通过 `GITHUB_REPOSITORY` 提供 `owner/repository`。

## 参数

| 参数                 | 说明                                           |
| -------------------- | ---------------------------------------------- |
| `repo`               | `owner/repository`；也可用 `GITHUB_REPOSITORY` |
| `release-tag`        | Release tag；直接 publish 时建议显式提供       |
| `release-title`      | 新建 Release 的标题                            |
| `release-draft`      | `true` 或 `1` 表示草稿                         |
| `release-prerelease` | `true` 或 `1` 表示预发布                       |

为保证可预测性，建议始终显式提供 `release-tag`。
