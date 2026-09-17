# Custom

The `custom` target allows you to run a custom script or command to publish your application artifact. This is useful when you need to integrate with a publishing platform that is not natively supported by fastforge.

## Usage

The `command` argument specifies the script or command to execute. The artifact path is made available to the command via the `ARTIFACT_PATH` environment variable. Any additional publish arguments are exposed as environment variables prefixed with `PUBLISH_ARG_` (uppercased, with dashes replaced by underscores).

### Command line

```
fastforge publish \
  --path dist/1.0.0+1/hello_world-1.0.0+1-android.apk \
  --targets custom \
  --publish-arg command=./scripts/publish.sh
```

### Configure `distribute_options.yaml`

```yaml
output: dist/
releases:
  - name: dev
    jobs:
      - name: release-dev-android
        package:
          platform: android
          target: apk
        publish:
          target: custom
          args:
            command: ./scripts/publish.sh
```

Run:

```
fastforge release --name dev
```

## Environment variables

The following environment variables are set when your command runs:

| Variable            | Description                                                                     |
| ------------------- | ------------------------------------------------------------------------------- |
| `ARTIFACT_PATH`     | Path to the artifact file to publish                                            |
| `PUBLISH_ARG_<KEY>` | Any additional publish argument, uppercased with dashes replaced by underscores |

## Example script

```sh
#!/bin/sh
set -e
echo "Publishing artifact: $ARTIFACT_PATH"
# Add your custom publish logic here
```
