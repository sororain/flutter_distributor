# fastforge

English | [简体中文](./README-ZH.md)

`fastforge` is the Rust CLI entry point of Fastforge. It orchestrates `analyze/build/package/publish/upgrade/workflow` subcommands.

## Quick Start

### 1. Requirements

- Rust stable (latest stable is recommended)
- Run commands from the repository root

### 2. Build and run

```bash
# From repository root
cargo build -p fastforge

# Show help
cargo run -p fastforge -- --help
```

### 3. Try the analyze command

```bash
# Analyze an app package. Supported: .aab/.apk/.ipa/.dmg
cargo run -p fastforge -- analyze /path/to/app.apk

# Write output to a JSON file
cargo run -p fastforge -- analyze /path/to/app.apk -o analysis.json
```

### 4. Try the build command

```bash
# Build Android APK directly (without package/publish)
cargo run -p fastforge -- build --platform android --target apk --build-flavor dev --build-dart-define APP_ENV=dev
```

### 5. Install as a local command

```bash
# Install from repository root
cargo install --path apps/cli

fastforge --help
```

### 6. Run a workflow locally

Create a workflow file under `.fastforge/workflows/`, for example `.fastforge/workflows/android.yml`:

```yaml
name: Android release

on:
  workflow_dispatch:
    inputs:
      flavor:
        description: Build flavor
        default: production

jobs:
  package:
    name: Package Android APK
    steps:
      - name: Build APK
        uses: fastforge/package
        with:
          platform: android
          target: apk
          output: dist/
          build-args: '{"build-flavor":"${{ inputs.flavor }}"}'
```

Run it from the project root:

```bash
# Run the only workflow found in .fastforge/workflows/
fastforge workflow run

# Run a specific workflow file
fastforge workflow run --file .fastforge/workflows/android.yml

# Pass workflow_dispatch inputs
fastforge workflow run --file .fastforge/workflows/android.yml --input flavor=staging

# Simulate another event or run against another workspace
fastforge workflow run --event push --workspace /path/to/project --file .fastforge/workflows/android.yml

# Inspect and validate workflows before running
fastforge workflow list --verbose
fastforge workflow validate .fastforge/workflows/android.yml
```

## Available subcommands

- `analyze`: Analyze application package metadata
- `build`: Build Flutter app outputs directly
- `package`: Package (command entry is in place)
- `publish`: Publish (command entry is in place)
- `upgrade`: Upgrade (command entry is in place)
- `workflow`: Run, list, and validate local workflows

## Development tips

```bash
# Check only this crate
cargo check -p fastforge_cli

# Run tests for this crate (if any)
cargo test -p fastforge_cli
```
