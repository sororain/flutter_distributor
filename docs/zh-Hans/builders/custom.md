# Custom Builder

[English](../../en/builders/custom.md) | 简体中文

Custom Builder 允许调用任意命令，并通过 glob 规则收集产物。当前模块已经实现，但尚未接入 `fastforge build`、`fastforge package` 或内置工作流 action。

## 构建器模型

自定义构建需要以下信息：

| 参数                | 必填 | 说明                             |
| ------------------- | :--: | -------------------------------- |
| `command`           |  是  | 要执行的程序或脚本               |
| `args`              |  否  | 字符串或字符串数组               |
| `output-directory`  |  是  | 产物根目录                       |
| `artifact-patterns` |  是  | 相对产物目录的 glob 字符串或数组 |

命令返回非零退出码，或者执行成功后没有匹配到任何产物时，构建都会失败。

## 当前使用建议

在顶层 CLI 接入完成前，请在本地工作流中使用普通 shell 步骤执行自定义构建，再把产物路径交给 `fastforge publish`。不要尝试使用尚不存在的 `--platform custom` 命令。
