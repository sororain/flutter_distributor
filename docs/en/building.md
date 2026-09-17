# Building

English | [简体中文](../zh-Hans/building.md)

Building invokes the build system used by the project and locates its raw artifacts. Build results can be used directly for testing and analysis, or passed to a [packager](packagers/README.md) to produce a distributable format.

## Building vs. Packaging

| Operation | Primary responsibility                               | Common outputs                                         |
| --------- | ---------------------------------------------------- | ------------------------------------------------------ |
| Build     | Compile the project and locate raw artifacts         | APK, AAB, IPA, `.app`, desktop bundle, web directory   |
| Package   | Invoke a builder, then prepare a distribution format | DMG, PKG, ZIP, or prepared APK, AAB, and IPA artifacts |

If you only need the final distributable file, use `fastforge package` directly. The packaging process invokes the appropriate builder automatically, so you do not need to run `fastforge build` first.

## Current Command Boundaries

Fastforge includes Gradle, Xcode, Flutter, and Custom builders, but they are exposed through the CLI in different ways:

| Builder              | Current entry point                             | Status                             |
| -------------------- | ----------------------------------------------- | ---------------------------------- |
| Gradle Android       | `fastforge package`, `fastforge/package` action | APK and AAB are connected          |
| Xcode iOS / macOS    | `fastforge/package` action                      | IPA and macOS `.app` are connected |
| Flutter Builder      | `fastforge build`; `package` for macOS only     | Multiple platforms connected       |
| Gradle Multiplatform | No top-level command                            | Build module only                  |
| Custom Builder       | No top-level command                            | Build module only                  |

> [!IMPORTANT]
> `fastforge build` is not currently a unified entry point for every builder. Do not use it to infer whether Gradle, Xcode, or Custom Builder is available; see the [builder overview](builders/README.md) for the current status.

## Run a Build Separately

The current top-level build command is:

```bash
fastforge build --platform <platform> [--target <target>]
```

For example:

```bash
fastforge build --platform android --target apk
fastforge build --platform web
```

`--platform` is required when the command runs, and some platforms also require `--target`. See [Flutter Builder](builders/flutter.md) for supported platforms, targets, build arguments, artifact locations, and host restrictions.

## Build Results

After a successful build, `fastforge build` writes JSON to standard output containing:

- `config`: build mode and effective arguments
- `platform`: target platform
- `outputDirectory`: build output directory
- `outputFiles`: detected artifact paths
- `duration`: build duration in milliseconds

If the build command succeeds but no artifact is found in the expected directory, Fastforge still reports a failure. This prevents later packaging or publishing steps from using an empty directory.

## Build During Packaging

Gradle and Xcode Builder are currently used mainly through `package`:

```bash
fastforge package --platform android --target apk
```

Xcode builds require arguments such as `project` and `scheme`. Pass them through a workflow action's `build-args`; see [Xcode Builder](builders/xcode.md) for a complete example.

## Next Steps

- Review builders and their integration status: [Builder Overview](builders/README.md)
- Produce a distributable artifact: [Packaging](packaging.md)
- Analyze a build artifact: [App Package Analysis](tools/analyze.md)
- Combine building, packaging, and publishing: [Local Workflows](workflows.md)
