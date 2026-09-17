# Custom（自定义）

`custom` 目标允许你运行自定义脚本或命令来发布应用制品。当你需要集成 fastforge 原生不支持的发布平台时，这非常有用。

## 用法

`command` 参数指定要执行的脚本或命令。制品路径通过 `ARTIFACT_PATH` 环境变量传递给命令。其他发布参数以 `PUBLISH_ARG_` 为前缀（大写，短横线替换为下划线）暴露为环境变量。

### 命令行

```
fastforge publish \
  --path dist/1.0.0+1/hello_world-1.0.0+1-android.apk \
  --targets custom \
  --publish-arg command=./scripts/publish.sh
```

### 配置 `distribute_options.yaml`

```yaml
output: dist/
releases:
  - name: dev
    jobs:
      - name: release-dev-android
        package:
          platform: android
          target: apk
        publish:
          target: custom
          args:
            command: ./scripts/publish.sh
```

运行：

```
fastforge release --name dev
```

## 环境变量

命令运行时会设置以下环境变量：

| 变量                | 说明                                   |
| ------------------- | -------------------------------------- |
| `ARTIFACT_PATH`     | 要发布的制品文件路径                   |
| `PUBLISH_ARG_<KEY>` | 其他发布参数，大写且短横线替换为下划线 |

## 示例脚本

```sh
#!/bin/sh
set -e
echo "正在发布制品: $ARTIFACT_PATH"
# 在此添加你的自定义发布逻辑
```
