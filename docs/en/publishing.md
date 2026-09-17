# Publishing

English | [简体中文](../zh-Hans/publishing.md)

`publish` sends an existing file or directory to a publishing target.

## Basic Usage

```bash
fastforge publish \
  --path dist/my-app.apk \
  --target fir \
  --publish-arg bundle_id=com.example.app
```

`--path` and `--target` are required. Pass publishing parameters with repeatable `--publish-arg KEY=VALUE` options:

```bash
fastforge publish \
  --path dist/app.zip \
  --target github \
  --publish-arg repo=owner/repository \
  --publish-arg release-tag=v1.0.0
```

See the [publisher overview](publishers/README.md) for available targets, parameters, and credential requirements. You can also inspect the current definitions in the `fastforge publish` command help.

## Credentials

Pass publishing credentials through environment variables on the running process. Do not place them in command-line arguments or commit them to the repository. Each publisher page lists the environment variables required by that target.

## Multi-step Automation

Use [Local Workflows](workflows.md) to combine build, package, publish, and shell commands. Put non-sensitive parameters in the action's `with` field, and continue to pass credentials through environment variables.
