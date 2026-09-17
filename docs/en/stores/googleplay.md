# Google Play

English | [简体中文](../../zh-Hans/stores/googleplay.md)

`fastforge googleplay` operates the Google Play Developer API directly, covering app verification, edits, AAB uploads, tracks, and catalogs.

## Authentication

`GOOGLE_PLAY_SERVICE_ACCOUNT_JSON` may contain complete service-account JSON or a file path:

```bash
export GOOGLE_PLAY_SERVICE_ACCOUNT_JSON="$PWD/service-account.json"
```

The service account needs Google Play Developer API access to the target app.

## Apps

```bash
fastforge googleplay app view com.example.myapp
fastforge googleplay app check com.example.myapp
```

## Edit Workflow

Most write operations take place within an edit:

```bash
fastforge googleplay edit create \
  --package-name com.example.myapp

fastforge googleplay edit commit \
  --package-name com.example.myapp \
  --edit-id <edit-id>
```

Use `edit delete` when an edit is no longer needed.

## Upload an AAB

Reuse an existing edit:

```bash
fastforge googleplay bundle upload dist/app-release.aab \
  --package-name com.example.myapp \
  --edit-id <edit-id>
```

You can also select a track and commit immediately after the upload:

```bash
fastforge googleplay bundle upload dist/app-release.aab \
  --package-name com.example.myapp \
  --track internal \
  --release-name '1.0.0 (1)' \
  --commit
```

## Tracks

```bash
fastforge googleplay track list \
  --package-name com.example.myapp \
  --edit-id <edit-id>

fastforge googleplay track view internal \
  --package-name com.example.myapp \
  --edit-id <edit-id>

fastforge googleplay track update internal \
  --package-name com.example.myapp \
  --edit-id <edit-id> \
  --version-code 1 \
  --status completed
```

## Catalog and Raw API

- See [Unified Catalog](catalog.md) for listing, image, and track metadata synchronization.
- Call endpoints without typed commands through `fastforge googleplay api`.
