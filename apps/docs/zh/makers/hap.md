# hap

将 Flutter 应用构建为 OpenHarmony HAP（HarmonyOS Ability Package）包。HAP 格式是 OpenHarmony 应用的基本部署单元，包含应用的资源、资产和编译后的代码，用于在 OpenHarmony 设备上安装。

## 环境要求

- 已安装 Flutter SDK 并配置了 OpenHarmony (OHOS) 工具链
- 已配置 OpenHarmony SDK 和开发环境
- 拥有用于签名的发布证书和 Profile

## 使用方法

运行：

```shell
fastforge package --platform ohos --targets hap
```

### 构建参数

OpenHarmony HAP 构建常用的 `build_args`：

| 参数 | 说明 |
|------|------|
| `dart-define` | Dart 环境变量 |
| `flavor` | 构建变体（例如 `dev`、`prod`） |

## 相关链接

- [申请发布证书](https://developer.huawei.com/consumer/cn/doc/app/agc-help-add-releasecert-0000001946273961)
- [申请发布 Profile](https://developer.huawei.com/consumer/cn/doc/app/agc-help-add-releaseprofile-0000001914714796)
- [OHOS Flutter 环境配置](https://gitcode.com/openharmony-tpc/flutter_flutter/tree/3.22.0-ohos)
