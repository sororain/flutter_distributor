# 发布

[English](../en/publishing.md) | 简体中文

`publish` 用于把已经存在的文件或目录发送到指定发布目标。

## 基本用法

```bash
fastforge publish \
  --path dist/my-app.apk \
  --target fir \
  --publish-arg bundle_id=com.example.app
```

`--path` 和 `--target` 为必填参数。发布参数使用可重复的 `--publish-arg KEY=VALUE`：

```bash
fastforge publish \
  --path dist/app.zip \
  --target github \
  --publish-arg repo=owner/repository \
  --publish-arg release-tag=v1.0.0
```

可用 target、各自的参数和凭证要求见[发布器总览](publishers/README.md)，也可以通过 `fastforge publish` 的命令提示确认当前定义。

## 凭证

发布凭证通过运行进程的环境变量传入，不要写入命令行参数或版本库。各 target 需要的环境变量见对应发布器页面。

## 多步骤自动化

需要组合构建、打包、发布和 shell 命令时，使用[本地工作流](workflows.md)。非敏感参数写入 action 的 `with` 字段，凭证仍通过环境变量传入。
