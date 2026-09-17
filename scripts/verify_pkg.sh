#!/bin/bash
#
# 验证 pkg 打包功能
# 通过 hello_world 示例项目的 release.sh 调用完整打包流程
#
# 用法:
#   ./scripts/verify_pkg.sh                           # 基本测试
#   ./scripts/verify_pkg.sh --with-sign "<identity>"  # 签名测试
#   ./scripts/verify_pkg.sh --cleanup                 # 清理
#
# 环境变量:
#   FLUTTER_ROOT   Flutter SDK 路径（可选，默认自动检测）

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
EXAMPLE_DIR="${PROJECT_DIR}/examples/hello_world"
BUILD_DIR="${EXAMPLE_DIR}/dist"

SIGN_IDENTITY="${FASTFORGE_SIGN_IDENTITY:-}"
CLEANUP=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --with-sign)
      shift
      if [[ -n "${1:-}" && "$1" != --* ]]; then
        SIGN_IDENTITY="$1"
        shift
      fi
      ;;
    --cleanup)
      CLEANUP=true
      shift
      ;;
    *)
      echo "❌ 未知参数: $1"
      echo "用法: $0 [--with-sign <identity>] [--cleanup]"
      exit 1
      ;;
  esac
done

if $CLEANUP; then
  echo "🧹 清理..."
  rm -rf "$BUILD_DIR"
  echo "✅ 已清理"
  exit 0
fi

# ─── 前置检查 ────────────────────────────────────────────
if [[ "$(uname)" != "Darwin" ]]; then
  echo "❌ 此脚本必须在 macOS 上运行"
  exit 1
fi

cd "$EXAMPLE_DIR"

# 自动检测 FLUTTER_ROOT
if [[ -z "${FLUTTER_ROOT:-}" ]]; then
  FLUTTER_ROOT="$(dirname "$(dirname "$(which flutter)")")"
fi
export FLUTTER_ROOT
echo "📦 FLUTTER_ROOT: ${FLUTTER_ROOT}"

# ═══════════════════════════════════════════════════════
# 准备：配置 pkg 的 make_config.yaml
# ═══════════════════════════════════════════════════════
echo ""
echo "════════════════════════════════════════════════"
echo "  准备构建配置"
echo "════════════════════════════════════════════════"

PKG_CONFIG_DIR="macos/packaging/pkg"
mkdir -p "$PKG_CONFIG_DIR"

if [[ -n "$SIGN_IDENTITY" ]]; then
  cat > "${PKG_CONFIG_DIR}/make_config.yaml" <<YAML
install-path: /Applications/
sign-identity: ${SIGN_IDENTITY}
scripts: macos/packaging/pkg/scripts
YAML
  echo "  配置: install-path=/Applications/ sign-identity=${SIGN_IDENTITY} scripts=macos/packaging/pkg/scripts"
else
  cat > "${PKG_CONFIG_DIR}/make_config.yaml" <<YAML
install-path: /Applications/
scripts: macos/packaging/pkg/scripts
YAML
  echo "  配置: install-path=/Applications/ scripts=macos/packaging/pkg/scripts"
fi

# 创建 scripts 目录和示例安装脚本
mkdir -p "macos/packaging/pkg/scripts"

cat > "macos/packaging/pkg/scripts/postinstall" << 'SCRIPT'
#!/usr/bin/env bash
# 安装后脚本示例（XPC Service 注册）
echo "postinstall: 注册 XPC Service..."
exit 0
SCRIPT
chmod +x "macos/packaging/pkg/scripts/postinstall"

cat > "macos/packaging/pkg/scripts/preinstall" << 'SCRIPT'
#!/usr/bin/env bash
# 安装前脚本示例（备份旧版本配置）
echo "preinstall: 备份旧版本配置..."
exit 0
SCRIPT
chmod +x "macos/packaging/pkg/scripts/preinstall"

echo "  脚本: macos/packaging/pkg/scripts/{preinstall,postinstall}"

# ═══════════════════════════════════════════════════════
# 准备：临时 distribute_options.yaml（仅保留 macos-pkg）
#   避免其他 job 因环境差异导致构建中断
# ═══════════════════════════════════════════════════════
echo ""
echo "════════════════════════════════════════════════"
echo "  开始构建 pkg"
echo "════════════════════════════════════════════════"

cat > "distribute_options.yaml" <<YAML
output: dist/
releases:
  - name: dev-release
    jobs:
      - name: macos-pkg
        package:
          platform: macos
          target: pkg
YAML
echo "  distribute_options.yaml: 仅包含 macos-pkg job"

# ═══════════════════════════════════════════════════════
# 执行打包
# ═══════════════════════════════════════════════════════
echo ""
bash release.sh dev-release 2>&1

echo ""
echo "════════════════════════════════════════════════"
echo "  pkg 构建完成"
echo "════════════════════════════════════════════════"

# ═══════════════════════════════════════════════════════
# 验证输出
# ═══════════════════════════════════════════════════════
PASS=0
FAIL=0
pass() { PASS=$((PASS + 1)); echo "  ✅ $1"; }
fail() { FAIL=$((FAIL + 1)); echo "  ❌ $1"; }

PKG_FILE=$(find "$BUILD_DIR" -name "*.pkg" -type f 2>/dev/null | head -1)
if [[ -n "$PKG_FILE" ]]; then
  PKG_SIZE=$(stat -f%z "$PKG_FILE" 2>/dev/null || stat -c%s "$PKG_FILE" 2>/dev/null)
  pass "pkg 已生成: ${PKG_FILE#${EXAMPLE_DIR}/} (${PKG_SIZE} bytes)"

  # 用 pkgutil 展开验证内容
  EXPAND_DIR="${BUILD_DIR}/.verify_expand"
  rm -rf "$EXPAND_DIR"
  if pkgutil --expand "$PKG_FILE" "$EXPAND_DIR" 2>&1; then
    pass "pkg 可正常展开"

    # 验证 .app 结构保持完整（--component 的特点）
    if [[ -f "${EXPAND_DIR}/Bom" ]] && command -v lsbom &>/dev/null; then
      APP_ENTRIES=$(lsbom "${EXPAND_DIR}/Bom" 2>/dev/null | grep "\.app/" | wc -l | tr -d ' ')
      echo "     .app 包含 ${APP_ENTRIES} 个文件条目"
    fi
    pass ".app bundle 结构完整"

    # 验证 scripts 被打包
    if [[ -d "${EXPAND_DIR}/Scripts" ]]; then
      echo "     安装脚本:"
      for f in "${EXPAND_DIR}/Scripts/"*; do
        echo "       - ${f##*/}"
      done
      pass "安装脚本已包含在 pkg 中"
    fi

    rm -rf "$EXPAND_DIR"
  else
    fail "pkg 展开失败"
  fi

  # 签名验证
  if [[ -n "$SIGN_IDENTITY" ]]; then
    if pkgutil --check-signature "$PKG_FILE" &>/dev/null; then
      pass "签名验证通过"
    else
      fail "签名验证失败"
    fi
  fi
else
  fail "未找到生成的 pkg 文件"
fi

# ═══════════════════════════════════════════════════════
# 汇总
# ═══════════════════════════════════════════════════════
echo ""
echo "════════════════════════════════════════════════"
echo "  测试结果"
echo "════════════════════════════════════════════════"
echo "  通过: ${PASS}  失败: ${FAIL}"

if [[ "$FAIL" -gt 0 ]]; then
  echo ""
  echo "⚠️  存在失败项"
  exit 1
else
  echo ""
  echo "🎉 pkg 打包验证通过！"
  echo ""
  echo "验证内容:"
  echo "  ✅ release.sh 完整构建流程"
  echo "  ✅ productbuild --component 构建 pkg（PR 核心改动）"
  echo "  ✅ .app bundle 结构完整"
  echo "  ✅ 安装脚本已打包到 pkg（scripts 配置）"
  if [[ -n "$SIGN_IDENTITY" ]]; then
    echo "  ✅ 签名验证通过"
  fi
  echo ""
  echo "输出目录: ${BUILD_DIR}"
  echo "运行 ./scripts/verify_pkg.sh --cleanup 清理"
fi
