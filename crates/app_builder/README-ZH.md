# app_builder

`app_builder` 是 Fastforge Flutter 构建编排层的 Rust 实现。

## 功能

- 与 Dart 版本一致的构建参数编码规则
- 与 Dart 版本一致的构建结果 JSON 结构
- 多平台 builder 注册与路由
- 从 `pubspec.yaml` 自动注入 `FLUTTER_BUILD_NAME` / `FLUTTER_BUILD_NUMBER`
- 支持 `FLUTTER_ROOT`

## 支持的构建目标

- `android/apk`
- `android/aab`
- `ios/ipa`（仅 macOS 主机）
- `macos`（仅 macOS 主机）
- `windows`（仅 Windows 主机）
- `linux`（仅 Linux 主机）
- `web`
- `ohos/hap`
- `ohos/app`

## API 使用示例

```rust
use app_builder::FlutterAppBuilder;
use serde_json::{Map, Value, json};

fn run_build() -> anyhow::Result<()> {
    let builder = FlutterAppBuilder::default();

    let mut args = Map::<String, Value>::new();
    args.insert("flavor".into(), Value::String("dev".into()));
    args.insert("dart-define".into(), json!({
        "APP_ENV": "dev"
    }));

    let result = builder.build("android", Some("apk"), args, None)?;
    println!("{}", result.to_json_compatible());

    Ok(())
}
```

## 参数编码规则

与 Dart 实现保持一致：

- `null` 或 `bool` => `--key`
- `string/number` => `--key value`
- `object/map` => 多次 `--key subKey=value`

## iOS 参数校验

当目标为 `ios/ipa` 时，以下参数至少传一个：

- `export-options-plist`
- `export-method`

## 运行测试

```bash
cargo test -p app_builder --offline
```

