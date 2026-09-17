#!/bin/sh
set -e

BINARY_NAME="fastforge"
INSTALL_DIR="${FASTFORGE_INSTALL_DIR:-/usr/local/bin}"
BINARY_PATH="${INSTALL_DIR}/${BINARY_NAME}"

# ── Terminal colors ────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

info()    { printf "${CYAN}info${RESET}  %s\n" "$1"; }
success() { printf "${GREEN}✓${RESET}     %s\n" "$1"; }
warn()    { printf "${YELLOW}warn${RESET}  %s\n" "$1"; }
error()   { printf "${RED}error${RESET} %s\n" "$1" >&2; exit 1; }

# ── Main ──────────────────────────────────────────────────────────────────────
main() {
  printf "\n${BOLD}fastforge uninstaller${RESET}\n\n"

  if [ ! -f "$BINARY_PATH" ]; then
    warn "${BINARY_NAME} not found at ${BINARY_PATH}. Nothing to uninstall."
    exit 0
  fi

  INSTALLED_VERSION="$("$BINARY_PATH" --version 2>/dev/null || true)"
  info "Found: ${INSTALLED_VERSION:-${BINARY_NAME}} at ${BINARY_PATH}"

  if [ -w "$BINARY_PATH" ]; then
    rm -f "$BINARY_PATH"
  else
    info "Requesting elevated permissions to remove ${BINARY_PATH}..."
    sudo rm -f "$BINARY_PATH"
  fi

  success "Removed ${BINARY_PATH}"

  printf "\n${BOLD}${GREEN}fastforge uninstalled successfully.${RESET}\n\n"
}

main
