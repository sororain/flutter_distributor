# 统一 Catalog

[English](../../en/stores/catalog.md) | 简体中文

Catalog 把应用商店元数据和图片同步到本地目录，便于版本控制、批量编辑和重复发布。

## 批量同步

先在 `.fastforge/config.yaml` 登记应用，然后运行：

```bash
fastforge store catalog pull
fastforge store catalog push
```

命令按配置顺序处理全部应用。单个应用失败不会中断后续应用；最终只要存在失败，进程就以错误状态退出。

## 默认目录

```text
.fastforge/stores/
├── appstore/
│   └── com.example.myapp/
└── googleplay/
    └── com.example.myapp/
```

## 单独同步 App Store

```bash
fastforge appstore catalog pull \
  --app com.example.myapp \
  --platform IOS \
  --output .fastforge/stores/appstore

fastforge appstore catalog push \
  --app com.example.myapp \
  --input .fastforge/stores/appstore \
  --dry-run
```

目录结构：

```text
<bundle-id>/
├── app.yaml
├── app_info.yaml
├── info/
├── versions/
│   └── IOS/
│       └── 1.0.0/
│           ├── version.yaml
│           └── en-US/
│               ├── localization.yaml
│               ├── screenshots/
│               └── previews/
└── .manifest.yaml
```

- `app_info.yaml` 管理主分类、次分类和子分类。
- `version.yaml` 保存版本级字段。
- `localization.yaml` 保存语言相关字段。
- `.manifest.yaml` 保存截图远端 ID 与本地校验信息。
- push 会按本地文件名处理截图顺序；同步前建议使用 `--dry-run`。

## 单独同步 Google Play

```bash
fastforge googleplay catalog pull \
  --package-name com.example.myapp \
  --output .fastforge/stores/googleplay

fastforge googleplay catalog push \
  --package-name com.example.myapp \
  --input .fastforge/stores/googleplay \
  --dry-run
```

目录结构：

```text
<package-name>/
├── app.yaml
├── listings/
├── screenshots/
│   └── <language>/
└── tracks/
```

Google Play 截图会按语言和图片类型继续分目录，包括 phone、7-inch、10-inch、TV 和 Wear。

## 安全操作顺序

1. `pull` 获取最新远端状态。
2. 在独立分支编辑 YAML 和图片。
3. 校验 diff，避免误删语言或截图。
4. 使用独立商店命令的 `push --dry-run` 预览。
5. 确认后执行实际 push。
