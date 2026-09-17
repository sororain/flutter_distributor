# Local Workflows

English | [简体中文](../zh-Hans/workflows.md)

`fastforge workflow` discovers, validates, and runs YAML workflows under `.fastforge/workflows/`. Use it to organize build, package, publish, and ordinary shell steps into repeatable tasks.

## Directory Structure

```text
your-project/
├── .fastforge/
│   └── workflows/
│       ├── android.yml
│       └── release.yml
└── project-files
```

## Minimal Workflow

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

## Discover Workflows

```bash
fastforge workflow list
fastforge workflow list --verbose
fastforge workflow list --dir /path/to/project
```

## Validate a Workflow

```bash
fastforge workflow validate .fastforge/workflows/release.yml
```

Validation parses only the workflow structure. It does not run commands or actions.

## Run a Workflow

When the directory contains only one workflow:

```bash
fastforge workflow run
```

When multiple workflows exist, select a file explicitly:

```bash
fastforge workflow run --file .fastforge/workflows/release.yml
```

Pass `workflow_dispatch` inputs:

```bash
fastforge workflow run \
  --file .fastforge/workflows/release.yml \
  --input flavor=staging \
  --input channel=beta
```

Simulate an event or change the working directory:

```bash
fastforge workflow run \
  --file .fastforge/workflows/release.yml \
  --event push \
  --workspace /path/to/project
```

## `fastforge/package` Action

Required inputs:

| Input      | Description     |
| ---------- | --------------- |
| `platform` | Target platform |
| `target`   | Package format  |

Optional inputs:

| Input           | Description                             |
| --------------- | --------------------------------------- |
| `output`        | Output directory; defaults to `dist/`   |
| `artifact-name` | Artifact name template                  |
| `skip-clean`    | Skip cleaning when the string is `true` |
| `build-target`  | Flutter Builder entry point             |
| `build-args`    | JSON object string                      |
| `hook-pre`      | Shell command to run before packaging   |
| `hook-post`     | Shell command to run after packaging    |

The active builder determines the fields accepted by `build-args`. See [Gradle Builder](builders/gradle.md), [Xcode Builder](builders/xcode.md), and [Flutter Builder](builders/flutter.md).

Example:

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

Action outputs:

- `artifact-count`
- `artifact-paths` (comma-separated)

## `fastforge/publish` Action

Required inputs:

| Input    | Description                  |
| -------- | ---------------------------- |
| `path`   | File or directory to publish |
| `target` | Publishing target            |

Publishing parameters can be grouped in JSON:

```yaml
- name: Publish
  uses: fastforge/publish
  with:
    path: dist/app.zip
    target: github
    publish-args: '{"repo":"owner/repository","release-tag":"v1.0.0"}'
```

If `publish-args` is omitted, every other `with` field except `path` and `target` becomes a publishing parameter:

```yaml
- name: Publish
  uses: fastforge/publish
  with:
    path: dist/app.zip
    target: github
    repo: owner/repository
    release-tag: v1.0.0
```

The action outputs `message`.

## Execution Result

The workflow engine creates execution layers from dependencies and reports job, step, command, and action status as it proceeds. Any failure makes the final result fail with a nonzero exit status, so workflows can be used directly from local scripts or CI.

## Notes

- `build-args` and `publish-args` must be valid JSON, not YAML objects.
- The built-in package action has the same current packaging coverage as the CLI.
- Pass publishing credentials through environment variables on the running process.
- When multiple workflows exist, use `--file` explicitly to avoid ambiguity.
