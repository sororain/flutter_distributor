# Packaging

English | [简体中文](../zh-Hans/packaging.md)

`fastforge package` prepares the project, runs the build, and turns its output into a distributable format.

```bash
fastforge package --platform android --target apk
```

Both `--platform` and `--target` are required, and one target is processed at a time. The example above applies to a native Gradle Android project. Flutter projects currently support only macOS formats, such as `--platform macos --target zip`. See the [packager overview](packagers/README.md) for platform formats and environment requirements.

## Packaging Process

A packaging run contains these stages:

1. Detect the build system from project files.
2. Invoke the appropriate platform tools to build the project.
3. Run the pre-package hook.
4. Produce the final distributable artifact.
5. Run the post-package hook.

Native projects without `pubspec.yaml` use Gradle for Android or Xcode for iOS and macOS. Flutter projects containing `pubspec.yaml` always use Flutter Builder.

## Current Support

| Build system    | Project type            | Platform | Formats             | Current entry point |
| --------------- | ----------------------- | -------- | ------------------- | ------------------- |
| Gradle          | Native Android          | Android  | `apk`, `aab`        | CLI, package action |
| Xcode           | Native iOS              | iOS      | `ipa`               | package action      |
| Xcode           | Native macOS            | macOS    | `dmg`, `pkg`, `zip` | package action      |
| Flutter Builder | Contains `pubspec.yaml` | macOS    | `dmg`, `pkg`, `zip` | CLI, package action |

> [!NOTE]
> `fastforge package` does not currently cover every build system equally. Flutter projects can package only macOS formats. Running `package` for `android` or `ios` finishes the build but fails because the packager is not connected. See the [builder overview](builders/README.md) for routing rules.

## Build Stage

The packaging command selects Gradle, Xcode, or Flutter Builder based on the project type, then passes detected artifacts to a packager. See the [builder overview](builders/README.md) for builder selection and integration status.

To inspect a Flutter Builder result separately, run:

```bash
fastforge build --platform <platform> [--target <target>]
```

See [Building](building.md) for platforms, targets, arguments, and output details.

## Packaging Options

### Skip Cleaning

To reuse the existing build cache, skip the cleaning step before packaging:

```bash
fastforge package --platform macos --target zip --skip-clean
```

### Custom Flutter Entry Point

```bash
fastforge package --platform macos --target dmg \
  --build-target lib/main_production.dart
```

### Output Directory

When `package` is run directly, it writes to `dist/`. To customize the directory or artifact name, use a workflow action's `output` and `artifact-name` fields.

## Lifecycle Hooks

```bash
fastforge package --platform macos --target zip \
  --hook-pre './scripts/before-package.sh' \
  --hook-post './scripts/after-package.sh'
```

Hooks run through the system shell. Fastforge provides these environment variables:

- `PLATFORM`
- `PACKAGE_FORMAT`
- `BUILD_MODE`
- `OUTPUT_DIRECTORY`
- `BUILD_OUTPUT_DIRECTORY`
- `BUILD_OUTPUT_FILES` (colon-separated)

Packaging fails immediately if any hook exits with a nonzero status.

## Automation

Use [Local Workflows](workflows.md) to combine multiple packaging targets, publishing operations, or other commands.
