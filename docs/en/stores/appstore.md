# App Store Connect

English | [简体中文](../../zh-Hans/stores/appstore.md)

`fastforge appstore` calls the App Store Connect API directly, covering app queries, build uploads, version submission, review submissions, and catalogs.

## Authentication

```bash
export APP_STORE_CONNECT_KEY_ID=ABC123DEFG
export APP_STORE_CONNECT_ISSUER_ID=00000000-0000-0000-0000-000000000000
export APP_STORE_CONNECT_KEY_PATH="$PWD/AuthKey_ABC123DEFG.p8"
```

All three variables are required. Store API commands use API Key authentication only.

## Apps

```bash
fastforge appstore app list
fastforge appstore app view com.example.myapp
```

`view` accepts either a bundle ID or an App Store app ID.

## Builds

```bash
fastforge appstore build upload dist/MyApp.ipa \
  --app com.example.myapp \
  --wait

fastforge appstore build list --app com.example.myapp
fastforge appstore build view <build-id>
fastforge appstore build wait <build-id> --timeout 30m
```

Uploads depend on macOS `xcrun`. `wait` waits for the App Store to finish processing the build.

## Versions

```bash
fastforge appstore version list --app com.example.myapp
fastforge appstore version view 1.0.0 --app com.example.myapp
fastforge appstore version submit 1.0.0 \
  --app com.example.myapp \
  --build <build-id> \
  --wait
```

`version submit` associates the build, creates a review submission, adds the version item, and submits it for review.

## Review Submissions

```bash
fastforge appstore submission list --app com.example.myapp
fastforge appstore submission create \
  --app com.example.myapp \
  --platform IOS
fastforge appstore submission items <submission-id>
fastforge appstore submission add-item <submission-id> \
  --item-type appStoreVersions \
  --item-id <version-id>
fastforge appstore submission submit <submission-id> --wait
fastforge appstore submission cancel <submission-id>
```

`list` can filter by `--platform` and `--state`. Run `add-item --help` to inspect supported review resource types.

## Catalog

See [Unified Catalog](catalog.md) for App Store metadata, categories, screenshots, and previews.

## Raw API

```bash
fastforge appstore api get --help
fastforge appstore api post --help
fastforge appstore api patch --help
fastforge appstore api delete --help
```

The raw API supports App Store Connect resources without typed commands. Automation scripts should prefer existing typed commands.
