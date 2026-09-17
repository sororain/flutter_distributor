# 本地工作流

[English](../en/workflows.md) | 简体中文

`fastforge workflow` 在本地发现、校验和运行 `.fastforge/workflows/` 下的 YAML 工作流。它适合把构建、打包、发布和普通 shell 步骤组织成可重复执行的任务。

## 目录结构

```text
your-project/
├── .fastforge/
│   └── workflows/
│       ├── android.yml
│       └── release.yml
└── project-files
```

## 最小工作流

```yaml
name: Android package

on:
  workflow_dispatch:
    inputs:
      flavor:
        description: Build flavor
        default: production

jobs:
  package:
    name: Package Android app
    steps:
      - name: Create APK
        uses: fastforge/package
        with:
          platform: android
          target: apk
          output: dist/
          build-args: '{"flavor":"${{ inputs.flavor }}"}'
```

## 发现工作流

```bash
fastforge workflow list
fastforge workflow list --verbose
fastforge workflow list --dir /path/to/project
```

## 校验工作流

```bash
fastforge workflow validate .fastforge/workflows/release.yml
```

校验只解析工作流结构，不执行其中的命令或 action。

## 运行工作流

当目录中只有一个工作流时：

```bash
fastforge workflow run
```

存在多个工作流时必须指定文件：

```bash
fastforge workflow run --file .fastforge/workflows/release.yml
```

传入 `workflow_dispatch` input：

```bash
fastforge workflow run \
  --file .fastforge/workflows/release.yml \
  --input flavor=staging \
  --input channel=beta
```

模拟事件或改变工作目录：

```bash
fastforge workflow run \
  --file .fastforge/workflows/release.yml \
  --event push \
  --workspace /path/to/project
```

## `fastforge/package` action

必填 input：

| Input      | 说明     |
| ---------- | -------- |
| `platform` | 目标平台 |
| `target`   | 打包格式 |

可选 input：

| Input           | 说明                           |
| --------------- | ------------------------------ |
| `output`        | 输出目录，默认 `dist/`         |
| `artifact-name` | 产物名称模板                   |
| `skip-clean`    | 字符串 `true` 时跳过构建前清理 |
| `build-target`  | Flutter Builder 的入口文件     |
| `build-args`    | JSON object 字符串             |
| `hook-pre`      | 打包前 shell 命令              |
| `hook-post`     | 打包后 shell 命令              |

`build-args` 的字段由实际构建器决定，分别见 [Gradle Builder](builders/gradle.md)、[Xcode Builder](builders/xcode.md)和 [Flutter Builder](builders/flutter.md)。

示例：

```yaml
- name: Package
  uses: fastforge/package
  with:
    platform: android
    target: aab
    output: artifacts/
    artifact-name: "my-app-{{build_name}}.{{ext}}"
    build-args: '{"flavor":"production","module":"app"}'
    hook-post: ./scripts/verify-artifact.sh
```

Action 输出：

- `artifact-count`
- `artifact-paths`（逗号分隔）

## `fastforge/publish` action

必填 input：

| Input    | 说明               |
| -------- | ------------------ |
| `path`   | 要发布的文件或目录 |
| `target` | 发布目标           |

可以把发布参数集中写成 JSON：

```yaml
- name: Publish
  uses: fastforge/publish
  with:
    path: dist/app.zip
    target: github
    publish-args: '{"repo":"owner/repository","release-tag":"v1.0.0"}'
```

如果省略 `publish-args`，除 `path` 和 `target` 外的其他 `with` 字段都会作为发布参数：

```yaml
- name: Publish
  uses: fastforge/publish
  with:
    path: dist/app.zip
    target: github
    repo: owner/repository
    release-tag: v1.0.0
```

Action 输出为 `message`。

## 执行结果

工作流引擎会按依赖关系生成执行层，逐步输出 job、step、命令和 action 状态。任何失败会使最终结果失败并以非零状态退出，适合直接用于本地脚本或 CI。

## 注意事项

- `build-args` 和 `publish-args` 必须是有效 JSON，不是 YAML object。
- 工作流内置 package action 与 CLI 使用相同的当前打包覆盖范围。
- 发布凭证应通过运行进程的环境变量传入。
- 多个工作流并存时显式使用 `--file`，避免选择歧义。
