# fastforge_core

Core traits and types for Fastforge — the foundational interfaces that all other crates implement.

## Status

Stable. This crate defines the shared abstractions for the entire Fastforge build and release pipeline.

## Provided Traits

| Trait | Purpose |
|---|---|
| `AppAnalyzer` | Analyze compiled application packages (APK, IPA, AAB, etc.) |
| `AppBuilder` | Orchestrate Flutter/Dart builds for various platforms |
| `AppPackager` | Package build outputs into distributable formats (APK, IPA, DMG, etc.) |
| `AppPublisher` | Upload and publish packages to distribution platforms |

## Provided Types

- **Config & Request:** `AnalyzeConfig`, `BuildConfig`, `BuildRequest`, `BuildMode`, `PackageConfig`, `PublishConfig`
- **Results & Errors:** `AnalyzeResult`, `AnalyzeError`, `BuildResult`, `BuildError`, `PackageResult`, `PackageError`, `PublishResult`, `PublishError`
- **Models:** `AppMetadata`, `Platform`
- **Callbacks:** `PublishProgressCallback`

## Usage

```rust
use fastforge_core::{AppAnalyzer, AppBuilder, AppPackager, AppPublisher};
```

Each trait is intended to be consumed generically — concrete implementations live in their respective crates (`fastforge_app_analyzer`, `fastforge_app_packager`, etc.).

## Run Tests

```bash
cargo test -p fastforge_core --offline
```
