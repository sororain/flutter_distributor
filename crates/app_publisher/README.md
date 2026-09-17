# fastforge_app_publisher

A unified application publisher — uploads and publishes app packages to various distribution platforms.

## Status

Work in progress (WIP). Multiple publisher backends are implemented or under development.

## Supported Providers

| Provider | Publisher | Status |
|---|---|---|
| GitHub Releases | `GitHubPublisher` | ✅ Implemented |
| Firebase App Distribution | `FirebasePublisher` | ✅ Implemented |
| Firebase Hosting | `FirebaseHostingPublisher` | ✅ Implemented |
| App Store Connect | `AppStorePublisher` | ✅ Implemented |
| Google Play Console | `PlayStorePublisher` | ✅ Implemented |
| Huawei AppGallery | `AppGalleryPublisher` | ✅ Implemented |
| PGYER | `PgyerPublisher` | ✅ Implemented |
| FIR.im | `FirPublisher` | ✅ Implemented |
| AWS S3 | `S3Publisher` | ✅ Implemented |
| Alibaba Cloud OSS | `OssPublisher` | ✅ Implemented |
| Tencent COS | `CosPublisher` | ✅ Implemented |
| Qiniu Kodo | `QiniuPublisher` | ✅ Implemented |
| Vercel | `VercelPublisher` | ✅ Implemented |
| Custom endpoint | `CustomPublisher` | ✅ Implemented |

## API Usage

```rust
use fastforge_app_publisher::GitHubPublisher;
use fastforge_core::{AppPublisher, PublishConfig};

fn publish_to_github() -> anyhow::Result<()> {
    let publisher = GitHubPublisher::new("owner/repo", "v1.0.0", "token");
    let config = PublishConfig::new("path/to/app.apk")?;
    let result = publisher.publish(&config, |progress| {
        println!("Progress: {}%", progress.percent);
    })?;

    println!("Published: {}", result.url);
    Ok(())
}
```

## CLI Usage

```bash
cargo run -p fastforge_app_publisher -- <provider> <options> <file>
```

## Run Tests

```bash
cargo test -p fastforge_app_publisher --offline
```
