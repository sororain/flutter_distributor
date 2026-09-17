# 开始

Fastforge 是一款全能的 Flutter 应用打包和发布工具，为您提供一站式解决方案，满足各种分发需求。

> **更名通知：** ~~Flutter Distributor~~ 已更名为 Fastforge。如果您之前使用的是 ~~Flutter Distributor~~，请注意所有功能保持不变，但包名、命令和文档已更新以反映此变更。

<div style="display: flex; flex-direction: row; gap: 10px;">
  <a href="https://github.com/fastforgedev/fastforge">
    <img
      alt="Fastforge on GitHub"
      src="https://img.shields.io/github/stars/fastforgedev/fastforge?style=for-the-badge&logo=GitHub"
    />
  </a>
  <a href="https://pub.dev/packages/fastforge">
    <img alt="Pub Likes" src="https://img.shields.io/pub/likes/fastforge?style=for-the-badge&logo=flutter&label=Pub%20Likes"/>
  </a>
  <a href="https://github.com/fastforgedev/fastforge/graphs/contributors">
    <img src="https://img.shields.io/github/all-contributors/fastforgedev/fastforge?style=for-the-badge" />
  </a>
</div>

## 主要特性

- 🚀 一键打包：支持 Android APK/AAB、iOS IPA、OpenHarmony HAP/APP 等多种格式
- 📦 多平台发布：支持 App Store、App Gallery、Google Play、Firebase、蒲公英、fir.im 等
- 🔄 CI/CD 集成：完美支持 GitHub Actions、GitLab CI 等持续集成平台
- 🛠 灵活配置：支持多环境、多 flavor、自定义构建参数

## 安装

```
dart pub global activate fastforge
```

> **Windows 用户请注意：** 激活后，请确保 pub 缓存 bin 目录已添加到 PATH 环境变量中：
> 1. 打开 **系统属性** → **高级** → **环境变量**
> 2. 在 **用户变量** 中找到 `Path`，点击 **编辑**
> 3. 添加 `%APPDATA%\Pub\Cache\bin` 并点击 **确定**
> 4. 重启终端，然后运行 `fastforge --help` 验证

## 用法

将 `distribute_options.yaml` 添加到你的项目根目录。

```yaml
output: dist/
```

### 配置一个发布器

以 `pgyer` 为例，登录后，点击右侧的用户头像，从菜单中进入[API 信息](https://www.pgyer.com/account/api)页面，复制 `API Key` 并将其添加到 `env` 节点。

```yaml
variables:
  PGYER_API_KEY: 'your api key'
```

Check out the [Publishers](/zh/publishers/appstore) documentation for all possible publishers and how to configure them.

### 配置发布项

下面的例子展示了如何添加一个包含打包 `apk`、`ipa` 包并发布到 `pgyer.com` 的发布项，一个发布项可以包含多个作业。

> `build_args` 是 `flutter build` 命令所支持的参数，请根据你的项目进行修改。

```yaml
releases:
  - name: dev
    jobs:
      # Build and publish your apk pkg to pgyer
      - name: release-dev-android
        package:
          platform: android
          target: apk
          build_args:
            flavor: dev
            target-platform: android-arm,android-arm64
            dart-define:
              APP_ENV: dev
        publish_to: pgyer
      # Build and publish your ipa pkg to pgyer
      - name: release-dev-ios
        package:
          platform: ios
          target: ipa
          build_args:
            flavor: dev
            export-options-plist: ios/dev_ExportOptions.plist
            dart-define:
              APP_ENV: dev
        publish_to: pgyer
```

### 完整的示例配置

```yaml
variables:
  PGYER_API_KEY: 'your api key'
output: dist/
releases:
  - name: dev
    jobs:
      # 构建 apk 包并将其发布到 pgyer
      - name: release-dev-android
        package:
          platform: android
          target: apk
          build_args:
            flavor: dev
            target-platform: android-arm,android-arm64
            dart-define:
              APP_ENV: dev
        publish_to: pgyer
      # 构建 ipa 包并将其发布到 pgyer
      - name: release-dev-ios
        package:
          platform: ios
          target: ipa
          build_args:
            flavor: dev
            export-options-plist: ios/dev_ExportOptions.plist
            dart-define:
              APP_ENV: dev
        publish_to: pgyer
```

### 发布你的应用

```
fastforge release --name dev
```

## 示例项目

Fastforge 包含几个示例项目，可帮助您快速上手：

- **[hello_world](https://github.com/fastforgedev/fastforge/tree/main/examples/hello_world)** - 演示核心功能的基础示例。
- **[multiple_flavors](https://github.com/fastforgedev/fastforge/tree/main/examples/multiple_flavors)** - 展示如何配置多种应用风格的示例。
- **[custom_binary_name](https://github.com/fastforgedev/fastforge/tree/main/examples/custom_binary_name)** - 如何自定义二进制输出名称的示例。

## 高级用法

### 环境变量

Fastforge 支持在配置文件中使用环境变量。这对于 API 密钥等敏感信息特别有用：

```yaml
variables:
  API_KEY: ${PGYER_API_KEY} # 使用 PGYER_API_KEY 环境变量
```

### CI/CD 集成

Fastforge 在 CI/CD 环境中运行良好。例如，使用 GitHub Actions：

```yaml
jobs:
  build-and-release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: subosito/flutter-action@v2
      - name: 安装 Fastforge
        run: dart pub global activate fastforge
      - name: 构建并发布
        run: fastforge release --name production
        env:
          API_KEY: ${{ secrets.API_KEY }}
```

查看[文档](https://fastforge.dev/zh/)获取更详细的 CI/CD 集成示例。

## 谢谢你

🎉 🎉 🎉
