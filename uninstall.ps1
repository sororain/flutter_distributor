# fastforge Windows uninstaller
# Usage:
#   iwr https://fastforge.dev/uninstall.ps1 | iex

$ErrorActionPreference = 'Stop'

$BinaryName = "fastforge"
$InstallDir = if ($env:FASTFORGE_INSTALL_DIR) { $env:FASTFORGE_INSTALL_DIR } else { "$env:LOCALAPPDATA\fastforge\bin" }

# ── Helpers ───────────────────────────────────────────────────────────────────
function Write-Info    { param($msg) Write-Host "info  $msg" -ForegroundColor Cyan }
function Write-Success { param($msg) Write-Host "✓     $msg" -ForegroundColor Green }
function Write-Warn    { param($msg) Write-Host "warn  $msg" -ForegroundColor Yellow }
function Write-Fail    { param($msg) Write-Host "error $msg" -ForegroundColor Red; exit 1 }

# ── Remove binary ─────────────────────────────────────────────────────────────
function Remove-Binary {
  $exePath = Join-Path $InstallDir "$BinaryName.exe"

  if (-not (Test-Path $exePath)) {
    Write-Warn "$BinaryName not found at $exePath. Nothing to uninstall."
    exit 0
  }

  try {
    $v = & $exePath --version 2>$null
    Write-Info "Found: $v at $exePath"
  } catch {
    Write-Info "Found: $exePath"
  }

  Remove-Item -Path $exePath -Force
  Write-Success "Removed $exePath"
}

# ── Remove install dir from PATH ──────────────────────────────────────────────
function Remove-FromUserPath {
  $currentPath = [System.Environment]::GetEnvironmentVariable("PATH", "User")
  if ($currentPath -like "*$InstallDir*") {
    Write-Info "Removing $InstallDir from user PATH..."
    $newPath = ($currentPath -split ';' | Where-Object { $_ -ne $InstallDir }) -join ';'
    [System.Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
    $env:PATH = ($env:PATH -split ';' | Where-Object { $_ -ne $InstallDir }) -join ';'
    Write-Success "PATH updated. Restart your terminal to apply changes."
  } else {
    Write-Info "$InstallDir was not found in your PATH."
  }
}

# ── Remove install dir if empty ───────────────────────────────────────────────
function Remove-InstallDirIfEmpty {
  if ((Test-Path $InstallDir) -and -not (Get-ChildItem -Path $InstallDir -ErrorAction SilentlyContinue)) {
    Remove-Item -Path $InstallDir -Force -Recurse -ErrorAction SilentlyContinue
    Write-Info "Removed empty directory: $InstallDir"
  }
}

# ── Main ──────────────────────────────────────────────────────────────────────
function Main {
  Write-Host ""
  Write-Host "fastforge uninstaller" -ForegroundColor White -BackgroundColor DarkRed
  Write-Host ""

  Remove-Binary
  Remove-FromUserPath
  Remove-InstallDirIfEmpty

  Write-Host ""
  Write-Host "fastforge uninstalled successfully." -ForegroundColor Green
  Write-Host ""
}

Main
