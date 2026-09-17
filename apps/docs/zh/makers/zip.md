# zip

将 Flutter 应用构建为 ZIP 存档 — 一种跨平台通用的压缩格式。ZIP 格式适用于将应用分发为便携式存档，用户可以解压后直接运行，无需安装程序。

## 环境要求

- `7z`（p7zip）— 一款高压缩率的文件归档工具

  macOS：

  ```bash
  brew install p7zip
  ```

  Ubuntu/Debian：

  ```bash
  sudo apt install p7zip-full
  ```

  Fedora：

  ```bash
  sudo dnf install p7zip
  ```

## 使用方法

运行：

```
fastforge package --platform linux --targets zip
```

输出的 ZIP 存档将包含编译后的应用程序及其依赖项，可直接解压使用。

## 相关链接

- [p7zip 官网](https://p7zip.sourceforge.net/)
- [构建和发布 Linux 应用程序](https://docs.flutter.dev/deployment/linux)
