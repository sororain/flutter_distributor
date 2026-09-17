# Builders

English | [简体中文](../../zh-Hans/builders/README.md)

Builders select a project's build tool, run the build command, and locate generated artifacts. Packagers consume build results; the two have distinct responsibilities.

## Current Builders

| Builder              | Platforms or targets                                  | CLI integration                                | Documentation                             |
| -------------------- | ----------------------------------------------------- | ---------------------------------------------- | ----------------------------------------- |
| Gradle Android       | Android APK, AAB                                      | Connected through `package`                    | [Gradle](gradle.md)                       |
| Gradle Multiplatform | Android, desktop, iOS framework                       | No top-level CLI integration                   | [Gradle](gradle.md#multiplatform-builder) |
| Xcode                | iOS IPA, macOS `.app`                                 | Connected to the `package` action              | [Xcode](xcode.md)                         |
| Flutter              | Android, iOS, macOS, Windows, Linux, Web, OpenHarmony | Connected to `build`; `package` for macOS only | [Flutter Builder](flutter.md)             |
| Custom               | User-defined commands and artifact rules              | No top-level CLI integration                   | [Custom Builder](custom.md)               |

“Implemented” and “connected to the CLI” are different states. A builder without a top-level entry point cannot be selected directly through `fastforge build`.

## Current Routing

`fastforge package` and the `fastforge/package` action select a build path based on the presence of `pubspec.yaml` in the project root and the value of `--platform`:

1. No `pubspec.yaml`, and the platform is `macos` or `ios`: select Xcode Builder.
2. No `pubspec.yaml`, and the platform is `android`: select Gradle Builder.
3. Otherwise, including every Flutter project with `pubspec.yaml`: select Flutter Builder.

> [!IMPORTANT]
> The Flutter Builder path is currently connected only to the macOS `dmg`, `pkg`, and `zip` packagers. Running `fastforge package --platform android` or `--platform ios` in a Flutter project completes the build, then fails because the corresponding packager is not connected. Use `fastforge build` to generate the raw artifact instead.

The current detection rules are intentionally simple. Run commands from the actual project root to avoid selecting the wrong builder.

## Choosing an Entry Point

- Native Android Gradle projects: use `fastforge package` or the workflow package action.
- Native iOS / macOS Xcode projects: use the workflow package action and pass project arguments through `build-args`.
- Flutter projects packaging macOS formats: use `fastforge package`; for other platforms, generate raw artifacts with `fastforge build`.
- Builders without CLI integration: continue to use the project's own build commands; do not assume the Fastforge CLI supports them yet.

See [Building](../building.md) for build commands and the result structure.
