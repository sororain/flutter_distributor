# zip

Build your Flutter app as a ZIP archive — a universal compression format that works across all platforms. The ZIP format is useful for distributing your application as a portable archive that users can extract and run without an installer.

## Requirements

- `7z` (p7zip) — a file archiver with high compression ratio

  macOS:

  ```bash
  brew install p7zip
  ```

  Ubuntu/Debian:

  ```bash
  sudo apt install p7zip-full
  ```

  Fedora:

  ```bash
  sudo dnf install p7zip
  ```

## Usage

Run:

```
fastforge package --platform linux --targets zip
```

The output ZIP archive will contain the compiled application and its dependencies, ready for extraction and use.

## Related Links

- [p7zip Homepage](https://p7zip.sourceforge.net/)
- [Build and release a Linux app](https://docs.flutter.dev/deployment/linux)
