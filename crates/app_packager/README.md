# fastforge_app_packager

A unified application packager — packages Flutter build outputs into platform-specific distribution formats.

## Status

Work in progress (WIP). Supports packaging for Android, iOS, macOS, Linux, Windows, Web, and OpenHarmony.

## Supported Formats

| Platform | Format | Packager |
|---|---|---|
| Android | APK | `AndroidApkPackager` |
| Android | AAB | `AndroidAabPackager` |
| iOS | IPA | `IOSIpaPackager` |
| macOS | DMG | `MacOSDmgPackager` |
| macOS | PKG | `MacOSPkgPackager` |
| macOS | ZIP | `MacOSZipPackager` |
| Linux | AppImage | `LinuxAppImagePackager` |
| Linux | DEB | `LinuxDebPackager` |
| Linux | RPM | `LinuxRpmPackager` |
| Linux | Pacman | `LinuxPacmanPackager` |
| Linux | ZIP | `LinuxZipPackager` |
| Linux | Direct | `LinuxDirectPackager` |
| Windows | EXE | `WindowsExePackager` |
| Windows | MSIX | `WindowsMsixPackager` |
| Windows | ZIP | `WindowsZipPackager` |
| Windows | Direct | `WindowsDirectPackager` |
| Web | ZIP | `WebZipPackager` |
| Web | Direct | `WebDirectPackager` |
| OpenHarmony | HAP | `OHOSHapPackager` |
| OpenHarmony | APP | `OHOSAppPackager` |

## API Usage

```rust
use fastforge_app_packager::AndroidApkPackager;
use fastforge_core::{AppPackager, PackageConfig};

fn package_apk() -> anyhow::Result<()> {
    let packager = AndroidApkPackager;
    let config = PackageConfig::new("build/app/outputs/flutter-apk")?;
    let result = packager.package(&config)?;

    println!("Package created at: {}", result.output_path);
    Ok(())
}
```

## CLI Usage

```bash
cargo run -p fastforge_app_packager -- <platform> <format> <input-dir> <output-dir>
```

## Run Tests

```bash
cargo test -p fastforge_app_packager --offline
```
