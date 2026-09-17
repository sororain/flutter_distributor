# fastforge

[English](./README.md) | 简体中文

`fastforge` 是 Fastforge 的 Rust 命令行入口，负责统一调度 `analyze/build/package/publish/upgrade/workflow` 子命令。

## 快速开始

### 1. 环境要求

- Rust stable（建议使用最新稳定版）
- 在仓库根目录执行命令

### 2. 编译与运行

```bash
# 在仓库根目录
cargo build -p fastforge

# 查看帮助
cargo run -p fastforge -- --help
```

### 3. 执行分析命令

```bash
# 分析应用包，支持: .aab/.apk/.ipa/.dmg
cargo run -p fastforge -- analyze /path/to/app.apk

# 输出到 JSON 文件
cargo run -p fastforge -- analyze /path/to/app.apk -o analysis.json
```

### 4. 执行构建命令

```bash
# 直接构建 Android APK（不打包/不发布）
cargo run -p fastforge -- build --platform android --target apk --build-flavor dev --build-dart-define APP_ENV=dev
```

### 5. 安装为本地命令

```bash
# 从仓库根目录安装
cargo install --path apps/cli

fastforge --help
```

### 6. 本地运行 workflow

在 `.fastforge/workflows/` 下创建 workflow 文件，例如 `.fastforge/workflows/android.yml`：

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

在项目根目录执行：

```bash
# 运行 .fastforge/workflows/ 中唯一的 workflow
fastforge workflow run

# 运行指定 workflow 文件
fastforge workflow run --file .fastforge/workflows/android.yml

# 传入 workflow_dispatch inputs
fastforge workflow run --file .fastforge/workflows/android.yml --input flavor=staging

# 模拟其他事件，或指定另一个工作目录
fastforge workflow run --event push --workspace /path/to/project --file .fastforge/workflows/android.yml

# 运行前查看和校验 workflow
fastforge workflow list --verbose
fastforge workflow validate .fastforge/workflows/android.yml
```

## 可用子命令

- `analyze`: 分析应用包元信息
- `build`: 直接构建 Flutter 应用产物
- `package`: 打包（命令入口已就绪）
- `publish`: 发布（命令入口已就绪）
- `upgrade`: 升级（命令入口已就绪）
- `workflow`: 本地运行、列出和校验 workflow

## 开发提示

```bash
# 只检查该 crate
cargo check -p fastforge_cli

# 运行该 crate 测试（如存在）
cargo test -p fastforge_cli
```
