use crate::GooglePlayContext;
use crate::cli::GlobalArgs;
use crate::cli::commands::edit::{create_edit, delete_edit};
use anyhow::{Context, Result};
use clap::Args;
use std::path::Path;

/// Google Play API 图片类型枚举（本地目录名 → API 参数值）
const IMAGE_TYPES: &[(&str, &str)] = &[
    ("phone_screenshots", "phoneScreenshots"),
    ("seven_inch_screenshots", "sevenInchScreenshots"),
    ("ten_inch_screenshots", "tenInchScreenshots"),
    ("tv_screenshots", "tvScreenshots"),
    ("wear_screenshots", "wearScreenshots"),
    ("feature_graphic", "featureGraphic"),
    ("tv_banner", "tvBanner"),
    ("icon", "icon"),
];

#[derive(Args, Debug)]
pub struct PullArgs {
    /// Package name of the app (e.g. "com.example.app")
    #[arg(long = "package-name")]
    pub package_name: String,

    /// Output directory root. Defaults to `.fastforge/stores/googleplay/{package_name}/`
    #[arg(long = "output")]
    pub output: Option<String>,
}

/// Ensure a directory exists.
fn ensure_dir(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)
        .with_context(|| format!("failed to create directory {}", path.display()))
}

/// Write a YAML value to a file.
fn write_yaml<T: serde::Serialize>(path: &Path, value: &T) -> Result<()> {
    let content = serde_yaml::to_string(value).context("failed to serialize YAML")?;
    std::fs::write(path, &content).with_context(|| format!("failed to write {}", path.display()))
}

/// Download a file from a URL and save it to disk.
async fn download_file(http: &reqwest::Client, url: &str, dest: &Path) -> Result<()> {
    let response = http
        .get(url)
        .send()
        .await
        .context("failed to download image")?
        .error_for_status()
        .context("download returned error")?;
    let bytes = response
        .bytes()
        .await
        .context("failed to read response body")?;
    tokio::fs::write(dest, &bytes)
        .await
        .with_context(|| format!("failed to write {}", dest.display()))?;
    Ok(())
}

/// Infer a file extension from a download URL.
fn ext_from_url(url: &str) -> &str {
    if url.contains(".png") || url.ends_with(".png") {
        "png"
    } else if url.contains(".jpg") || url.contains(".jpeg") || url.ends_with("jpg") {
        "jpg"
    } else if url.contains(".webp") || url.ends_with("webp") {
        "webp"
    } else {
        "png"
    }
}

pub async fn execute(args: &PullArgs, _global: &GlobalArgs) -> Result<()> {
    let ctx = GooglePlayContext::from_env().await?;
    execute_with_context(args, &ctx).await
}

/// Pull catalog data using an existing Google Play context.
pub async fn execute_with_context(args: &PullArgs, ctx: &GooglePlayContext) -> Result<()> {
    let start = std::time::Instant::now();
    let mut pulled_count = 0u64;
    let package_name = &args.package_name;

    let output_root = args
        .output
        .as_deref()
        .map(Path::new)
        .unwrap_or_else(|| Path::new(".fastforge/stores/googleplay"))
        .join(package_name);
    ensure_dir(&output_root)?;

    // 1. Write app.yaml
    let app_yaml = serde_json::json!({
        "packageName": package_name,
    });
    write_yaml(&output_root.join("app.yaml"), &app_yaml)?;
    eprintln!("  ✓ app.yaml");
    pulled_count += 1;

    // 2. Create a temporary edit for reading
    eprintln!("📦 Creating temporary edit...");
    let edit_id = create_edit(ctx, package_name)
        .await
        .context("failed to create edit for reading")?;

    // 3. Fetch listings
    eprintln!("📝 Fetching store listings...");
    let listings_resp = ctx
        .client
        .androidpublisher_edits_listings_list(
            package_name,
            &edit_id,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to fetch listings: {e}"))?;
    let listings = listings_resp.into_inner().listings;

    let listings_dir = output_root.join("listings");
    ensure_dir(&listings_dir)?;

    let mut listing_languages: Vec<String> = Vec::new();
    for listing in &listings {
        let language = listing
            .language
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        let listing_path = listings_dir.join(format!("{language}.yaml"));
        write_yaml(&listing_path, listing)?;
        eprintln!("  ✓ listings/{language}.yaml");
        listing_languages.push(language);
        pulled_count += 1;
    }

    eprintln!(
        "  Found {} languages: {:?}",
        listing_languages.len(),
        listing_languages
    );

    // 4. Fetch screenshots & images for each listing language
    eprintln!("🖼️  Fetching screenshots and images...");
    for language in &listing_languages {
        for (dir_name, api_type) in IMAGE_TYPES {
            let images_resp = ctx
                .client
                .androidpublisher_edits_images_list(package_name, &edit_id, language, api_type)
                .await;

            let images = match images_resp {
                Ok(resp) => resp.into_inner().images,
                Err(e) => {
                    eprintln!("  ⚠ skipped ({language}/{api_type}): {e}");
                    continue;
                }
            };

            if images.is_empty() {
                continue;
            }

            let image_dir = output_root
                .join("screenshots")
                .join(language)
                .join(dir_name);
            ensure_dir(&image_dir)?;

            for (idx, image) in images.iter().enumerate() {
                let image_id = image.id.as_deref().unwrap_or("unknown");
                let image_url = image.url.as_deref().unwrap_or("");
                if image_url.is_empty() {
                    continue;
                }

                let ext = ext_from_url(image_url);
                let filename = format!("{:03}_{}.{}", idx + 1, image_id, ext);
                let dest = image_dir.join(&filename);

                match download_file(&ctx.http, image_url, &dest).await {
                    Ok(_) => {
                        eprintln!("  ✓ screenshots/{language}/{dir_name}/{filename}");
                        pulled_count += 1;
                    }
                    Err(e) => {
                        eprintln!("  ⚠ failed to download {language}/{dir_name}/{image_id}: {e}");
                    }
                }
            }
        }
    }

    // 5. Fetch tracks
    eprintln!("📦 Fetching tracks...");
    let tracks_resp = ctx
        .client
        .androidpublisher_edits_tracks_list(
            package_name,
            &edit_id,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to fetch tracks: {e}"))?;
    let tracks = tracks_resp.into_inner().tracks;

    let tracks_dir = output_root.join("tracks");
    ensure_dir(&tracks_dir)?;

    for track in &tracks {
        let track_name = track.track.clone().unwrap_or_else(|| "unknown".to_string());
        // Sanitize track name for filename (e.g. "wear:production" -> "wear_production")
        let filename = track_name.replace(':', "_");
        let track_path = tracks_dir.join(format!("{filename}.yaml"));
        write_yaml(&track_path, track)?;
        eprintln!("  ✓ tracks/{filename}.yaml");
        pulled_count += 1;
    }

    // 6. Delete the temporary edit (cleanup)
    delete_edit(ctx, package_name, &edit_id)
        .await
        .context("failed to clean up temporary edit")?;

    let elapsed = start.elapsed();
    eprintln!(
        "\n✅ Pull complete: {pulled_count} resources synced to {} in {elapsed:?}",
        output_root.display()
    );
    Ok(())
}
