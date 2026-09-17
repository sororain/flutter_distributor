# Custom

English | [简体中文](../../zh-Hans/publishers/custom.md)

The `custom` target connects services without a built-in publisher through a custom shell command.

## Usage

```bash
fastforge publish --path dist/app.zip --target custom \
  --publish-arg 'command=./scripts/upload.sh' \
  --publish-arg channel=stable
```

macOS and Linux use `sh -c`; Windows uses `cmd /C`.

## Environment Variables

The custom command can read:

- `ARTIFACT_PATH`: current artifact path
- `PUBLISH_ARG_<KEY>`: publishing arguments other than `command`

Argument keys are converted to uppercase and non-alphanumeric characters are replaced with underscores. For example, `release-channel` becomes `PUBLISH_ARG_RELEASE_CHANNEL`.

Publishing fails if the command exits with a nonzero status. On success, standard output becomes the publishing result's `message`.
