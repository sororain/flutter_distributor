#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
OUTPUT_DIR="$SCRIPT_DIR/out"

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "This script only works on macOS." >&2
  exit 1
fi

require_cmd cargo
require_cmd hdiutil
require_cmd cp
require_cmd du
require_cmd sips
require_cmd plutil
require_cmd tiffutil
require_cmd xattr

mkdir -p "$OUTPUT_DIR"

build_example() {
  local config_name="$1"
  local output_name="$2"
  local config_path="$SCRIPT_DIR/$config_name"
  local output_path="$OUTPUT_DIR/$output_name"

  rm -f "$output_path"
  echo "Building $config_name -> out/$output_name"
  (
    cd "$REPO_ROOT"
    cargo run -p dmg_maker -- "$config_path" "$output_path"
  )
}

build_example "standard.json" "standard.dmg"
build_example "background_color.json" "background_color.dmg"
build_example "window.json" "window.dmg"
build_example "format_filesystem.json" "format_filesystem.dmg"
build_example "compat_legacy.json" "compat_legacy.dmg"

echo "Build outputs:"
find "$OUTPUT_DIR" -maxdepth 1 -name '*.dmg' -print | sort
