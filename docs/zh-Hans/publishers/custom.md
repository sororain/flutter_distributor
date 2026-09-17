# Custom

[English](../../en/publishers/custom.md) | 简体中文

`custom` target 通过自定义 shell 命令接入没有内置发布器的服务。

## 用法

```bash
fastforge publish --path dist/app.zip --target custom \
  --publish-arg 'command=./scripts/upload.sh' \
  --publish-arg channel=stable
```

macOS / Linux 使用 `sh -c`，Windows 使用 `cmd /C`。

## 环境变量

自定义命令可读取：

- `ARTIFACT_PATH`：当前产物路径
- `PUBLISH_ARG_<KEY>`：除 `command` 外的发布参数

参数键会转为大写，非字母数字字符替换为下划线。例如 `release-channel` 会成为 `PUBLISH_ARG_RELEASE_CHANNEL`。

命令返回非零状态时发布失败。成功时，标准输出内容会成为发布结果 message。
