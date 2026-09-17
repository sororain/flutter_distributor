# Gradle Builder

English | [简体中文](../../zh-Hans/builders/gradle.md)

Gradle Builder currently builds APK and AAB artifacts for Android projects. It prefers the Gradle Wrapper in the project root and falls back to `gradle` from `PATH` when no wrapper is found.

## Current Integration

| Type                  | Target                       | CLI status                   |
| --------------------- | ---------------------------- | ---------------------------- |
| Android               | `apk`                        | Connected through `package`  |
| Android               | `aab`                        | Connected through `package`  |
| Multiplatform Android | `android-apk`, `android-aab` | No top-level CLI integration |
| Multiplatform Desktop | `desktop`                    | No top-level CLI integration |
| Multiplatform iOS     | `ios-framework`              | No top-level CLI integration |

## Android Builds

When packaging directly, Fastforge runs the Gradle build automatically:

```bash
fastforge package --platform android --target apk
fastforge package --platform android --target aab
```

The default build uses the Release variant. The Gradle task is generated from the target, flavor, and module:

| Target | No flavor         | `dev` flavor         |
| ------ | ----------------- | -------------------- |
| `apk`  | `assembleRelease` | `assembleDevRelease` |
| `aab`  | `bundleRelease`   | `bundleDevRelease`   |

When a module is specified, the task receives a module prefix, such as `:androidApp:assembleDevRelease`.

## Workflow Arguments

To pass a flavor, module, or Gradle property, use the package action's `build-args`:

```yaml
- name: Package Android APK
  uses: fastforge/package
  with:
    platform: android
    target: apk
    output: artifacts/
    build-args: '{"flavor":"dev","module":"app"}'
```

The builder recognizes these arguments:

| Argument          | Description                                         |
| ----------------- | --------------------------------------------------- |
| `flavor`          | Android product flavor                              |
| `profile`         | Use Profile when this key exists; otherwise Release |
| `module`          | Gradle module name                                  |
| `gradle-property` | JSON object converted to `-Pkey=value`              |
| `system-property` | JSON object converted to `-Dkey=value`              |

`build-args` must be a JSON object string.

## Artifact Locations

| Target | Default search location     |
| ------ | --------------------------- |
| APK    | `app/build/outputs/apk/`    |
| AAB    | `app/build/outputs/bundle/` |

If the build command succeeds but no file with the expected extension is found, Fastforge treats the build as failed.

## Multiplatform Builder

The build module also contains builders for Android APK/AAB, desktop distributions for the current host, and iOS XCFramework. They are not connected to `fastforge build`, `fastforge package`, or a built-in workflow action, so this page does not provide directly runnable Fastforge commands for them.
