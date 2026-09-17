# Firebase

English | [简体中文](../../zh-Hans/publishers/firebase.md)

Fastforge provides two Firebase targets, both of which depend on the Firebase CLI installed on the system.

## App Distribution

Target: `firebase`

```bash
export FIREBASE_TOKEN=firebase-token

fastforge publish --path dist/app.apk --target firebase \
  --publish-arg app=1:1234567890:android:abcdef
```

`app` is required. Optional arguments are passed directly to the Firebase CLI:

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

Target: `firebase-hosting`

`--path` should point to the directory to deploy:

```bash
export FIREBASE_PROJECT_ID=my-project

fastforge publish --path build/web --target firebase-hosting
```

You can override the environment variable with the `project-id` argument:

```bash
fastforge publish --path build/web --target firebase-hosting \
  --publish-arg project-id=my-project
```

Fastforge generates `.firebaserc` and `firebase.json` in the target directory, then runs `firebase deploy`. The token may be omitted when the Firebase CLI is already signed in; setting `FIREBASE_TOKEN` is recommended in CI.
