# Vercel

[English](../../en/publishers/vercel.md) | 简体中文

`vercel` target 把目录部署到 Vercel Production，依赖系统中的 Vercel CLI。

## 配置

```bash
export VERCEL_ORG_ID=team_or_user_id
export VERCEL_PROJECT_ID=project_id
```

请先确保 Vercel CLI 已登录，或已通过其支持的方式配置 token。

## 发布

```bash
fastforge publish --path build/web --target vercel
```

Fastforge 会在目标目录生成 `.vercel/project.json`，然后运行：

```text
vercel --prod
```

也可以通过 `org-id`、`project-id` 发布参数覆盖环境变量。
