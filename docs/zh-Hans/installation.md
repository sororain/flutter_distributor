# 安装

[English](../en/installation.md) | 简体中文

Fastforge CLI 以独立二进制运行。CLI 本身不附带各平台的 SDK、构建工具、签名工具或第三方发布客户端。

## 使用安装脚本

### macOS / Linux

```bash
curl -fsSL https://raw.githubusercontent.com/fastforgedev/fastforge/main/install.sh | sh
```

默认安装到 `/usr/local/bin/fastforge`。可以指定安装目录或版本：

```bash
curl -fsSL https://raw.githubusercontent.com/fastforgedev/fastforge/main/install.sh \
  | FASTFORGE_INSTALL_DIR="$HOME/.local/bin" sh

curl -fsSL https://raw.githubusercontent.com/fastforgedev/fastforge/main/install.sh \
  | FASTFORGE_VERSION="0.7.0" sh
```

安装脚本支持：

- macOS：Apple Silicon、Intel
- Linux：AArch64、x86_64（GNU libc）

如果目标目录不可写，脚本会请求 `sudo` 权限。

### Windows

在 PowerShell 中执行：

```powershell
iwr https://raw.githubusercontent.com/fastforgedev/fastforge/main/install.ps1 | iex
```

默认安装到 `%LOCALAPPDATA%\fastforge\bin`，并写入当前用户的 `PATH`。可以通过环境变量指定版本或目录：

```powershell
$env:FASTFORGE_VERSION = "0.7.0"
$env:FASTFORGE_INSTALL_DIR = "$HOME\bin"
iwr https://raw.githubusercontent.com/fastforgedev/fastforge/main/install.ps1 | iex
```

## 从源码安装

从源码安装需要最新稳定版 Cargo 工具链：

```bash
git clone https://github.com/fastforgedev/fastforge.git
cd fastforge
cargo install --path apps/cli
```

开发时可以不安装，直接在仓库根目录运行：

```bash
cargo run -p fastforge_cli -- --help
```

## 验证安装

```bash
fastforge --version
fastforge --help
```

如果终端找不到命令，请确认安装目录已加入 `PATH`。

## 项目工具链

根据实际操作准备依赖：

| 场景               | 额外依赖                                                    |
| ------------------ | ----------------------------------------------------------- |
| Android 构建或分析 | Android SDK；APK 分析需要 `ANDROID_HOME` 下可用的 `aapt2`   |
| AAB 分析           | Android SDK 的 `aapt2`，或通过 `BUNDLETOOL` 指定 bundletool |
| iOS / macOS 构建   | macOS、Xcode 和命令行工具                                   |
| Flutter 构建       | Flutter SDK 和目标平台工具链，且 `flutter` 位于 `PATH`      |
| App Store 上传     | macOS、`xcrun` 和有效的 App Store Connect 凭证              |
| Firebase 发布      | Firebase CLI                                                |
| Vercel 发布        | Vercel CLI                                                  |

## 卸载

macOS / Linux：

```bash
curl -fsSL https://raw.githubusercontent.com/fastforgedev/fastforge/main/uninstall.sh \
  | sh
```

Windows：

```powershell
iwr https://raw.githubusercontent.com/fastforgedev/fastforge/main/uninstall.ps1 | iex
```

若安装时使用了 `FASTFORGE_INSTALL_DIR`，卸载时应传入相同的值。
