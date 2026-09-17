# Firebase

[English](../../en/publishers/firebase.md) | 简体中文

Fastforge 提供两个 Firebase target，均依赖系统中的 Firebase CLI。

## App Distribution

Target：`firebase`

```bash
export FIREBASE_TOKEN=firebase-token

fastforge publish --path dist/app.apk --target firebase \
  --publish-arg app=1:1234567890:android:abcdef
```

`app` 为必填参数。可选参数会直接传给 Firebase CLI：

- `release-notes`
- `release-notes-file`
- `testers`
- `testers-file`
- `groups`
- `groups-file`

```bash
fastforge publish --path dist/app.apk --target firebase \
  --publish-arg app=1:1234567890:android:abcdef \
  --publish-arg groups=qa-team \
  --publish-arg 'release-notes=Internal build'
```

## Firebase Hosting

Target：`firebase-hosting`

`--path` 应指向需要部署的目录：

```bash
export FIREBASE_PROJECT_ID=my-project

fastforge publish --path build/web --target firebase-hosting
```

也可以用 `project-id` 参数覆盖环境变量：

```bash
fastforge publish --path build/web --target firebase-hosting \
  --publish-arg project-id=my-project
```

Fastforge 会在目标目录生成 `.firebaserc` 和 `firebase.json`，随后运行 `firebase deploy`。已登录 Firebase CLI 时 token 可省略；CI 中建议设置 `FIREBASE_TOKEN`。
