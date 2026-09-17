# GitHub Releases

English | [简体中文](../../zh-Hans/publishers/github.md)

The `github` target uploads a file to a GitHub Release and creates the release if it does not exist.

## Authentication

```bash
export GITHUB_TOKEN=github-token
```

The token needs read and write access to releases in the target repository.

## Publish

```bash
fastforge publish --path dist/app.zip --target github \
  --publish-arg repo=owner/repository \
  --publish-arg release-tag=v1.0.0
```

In CI, `owner/repository` can be supplied through `GITHUB_REPOSITORY`.

## Arguments

| Argument             | Description                                           |
| -------------------- | ----------------------------------------------------- |
| `repo`               | `owner/repository`; may also use `GITHUB_REPOSITORY`  |
| `release-tag`        | Release tag; provide explicitly for direct publishing |
| `release-title`      | Title for a newly created release                     |
| `release-draft`      | `true` or `1` creates a draft                         |
| `release-prerelease` | `true` or `1` creates a prerelease                    |

For predictable behavior, always provide `release-tag` explicitly.
