# Getting Started

English | [简体中文](../zh-Hans/getting-started.md)

This page starts with installation verification and introduces Fastforge entry points for packaging, publishing, and artifact analysis. Platform-specific details are covered in the corresponding packager documentation.

## 1. Check the Environment

```bash
fastforge --version
fastforge --help
```

Run commands from the project root, and install the SDKs, build tools, and signing tools required by the target platform in advance.

## 2. Choose a Platform and Format

`package` builds the platform project and prepares a distributable artifact. Availability depends on the project type.

Native Gradle Android projects can package APK or AAB artifacts directly:

```bash
fastforge package --platform android --target apk
fastforge package --platform android --target aab
```

Flutter projects currently support only macOS packaging formats:

```bash
fastforge package --platform macos --target dmg
```

For Android, iOS, and other Flutter targets, generate the artifact separately with `fastforge build`; see [Building](building.md). iOS and macOS Xcode projects must pass project parameters through a workflow; see [Xcode Builder](builders/xcode.md). Before running a command, review the [builder overview](builders/README.md) and [packager overview](packagers/README.md) to confirm current coverage.

## 3. Publish an Existing Artifact

```bash
fastforge publish \
  --path dist/app.apk \
  --target fir
```

See the [publisher overview](publishers/README.md) for publishing parameters and credentials. If you already have an artifact, you can skip packaging and publish or analyze it directly.

## 4. Analyze an Artifact

`analyze` selects an analyzer based on the file extension and writes JSON to standard output:

```bash
fastforge analyze dist/app.apk
```

Write the result to a file:

```bash
fastforge analyze dist/app.apk \
  --output analysis.json
```

Supported inputs are `.apk`, `.aab`, `.ipa`, `.dmg`, and macOS `.app` bundles. DMG and `.app` analysis are available only on macOS.

## Next Steps

- Run a build separately: [Building](building.md)
- Create an installer or package: [Packaging](packaging.md)
- Upload an existing artifact: [Publishing](publishing.md)
- Run automation tasks: [Local Workflows](workflows.md)
- Analyze app artifacts: [App Package Analysis](tools/analyze.md)
- Manage store metadata: [Store Management](stores/README.md)
