use crate::GooglePlayContext;
use crate::cli::GlobalArgs;
use crate::cli::commands::edit::{commit_edit, create_edit};
use crate::client::types::{Listing, Track};
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
pub struct PushArgs {
    /// Package name of the app (e.g. "com.example.app")
    #[arg(long = "package-name")]
    pub package_name: String,

    /// Input directory root. Defaults to `.fastforge/stores/googleplay/{package_name}/`
    #[arg(long = "input")]
    pub input: Option<String>,

    /// Dry-run mode: show what would be pushed without making changes
    #[arg(long = "dry-run", default_value_t = false)]
    pub dry_run: bool,
}

#[derive(Debug)]
struct PushAction {
    resource_type: String,
    language: Option<String>,
    action: &'static str,
    details: String,
}

/// Read a YAML file and deserialize it.
fn read_yaml<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    serde_yaml::from_str(&content).with_context(|| format!("failed to parse {}", path.display()))
}

/// Collect listing YAML files from a directory.
fn listing_files(listings_dir: &Path) -> Result<Vec<(String, std::path::PathBuf)>> {
    let mut files = Vec::new();
    if !listings_dir.exists() {
        return Ok(files);
    }
    for entry in std::fs::read_dir(listings_dir).context("reading listings")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("yaml") {
            let language = path
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            if !language.is_empty() {
                files.push((language, path));
            }
        }
    }
    Ok(files)
}

/// Collect track YAML files from a directory.
fn track_files(tracks_dir: &Path) -> Result<Vec<(String, std::path::PathBuf)>> {
    let mut files = Vec::new();
    if !tracks_dir.exists() {
        return Ok(files);
    }
    for entry in std::fs::read_dir(tracks_dir).context("reading tracks")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("yaml") {
            let track_name = path
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            if !track_name.is_empty() {
                files.push((track_name, path));
            }
        }
    }
    Ok(files)
}

/// Collect screenshot/image files for a given type directory.
fn screenshot_files(screenshots_type_dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();
    if !screenshots_type_dir.exists() {
        return Ok(files);
    }
    for entry in std::fs::read_dir(screenshots_type_dir).context("reading screenshots")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file()
            && let Some(name) = path.file_name().and_then(|n| n.to_str())
            && !name.starts_with('.')
        {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

/// Infer MIME type from file extension.
fn infer_mime_type(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase()
        .as_str()
    {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        _ => "image/png",
    }
}

/// Recursively discover all screenshot/image push actions.
fn discover_screenshot_actions(base_dir: &Path, actions: &mut Vec<PushAction>) -> Result<()> {
    let screenshots_dir = base_dir.join("screenshots");
    if !screenshots_dir.exists() {
        return Ok(());
    }

    for lang_entry in std::fs::read_dir(&screenshots_dir).context("reading screenshots")? {
        let lang_entry = lang_entry?;
        let lang_path = lang_entry.path();
        if !lang_path.is_dir() {
            continue;
        }
        let language = lang_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        if language.is_empty() {
            continue;
        }

        for (dir_name, _api_type) in IMAGE_TYPES {
            let type_dir = lang_path.join(dir_name);
            let files = screenshot_files(&type_dir)?;
            if !files.is_empty() {
                actions.push(PushAction {
                    resource_type: "images".into(),
                    language: Some(language.clone()),
                    action: "upload",
                    details: format!("screenshots/{language}/{dir_name}/ ({} files)", files.len()),
                });
            }
        }
    }
    Ok(())
}

/// Execute screenshot/images push using the generated client.
async fn push_images(
    ctx: &GooglePlayContext,
    package_name: &str,
    edit_id: &str,
    base_dir: &Path,
    language: &str,
    dir_name: &str,
    api_type: &str,
) -> Result<()> {
    let type_dir = base_dir.join("screenshots").join(language).join(dir_name);

    let files = screenshot_files(&type_dir)?;
    if files.is_empty() {
        return Ok(());
    }

    // Delete existing images for this language/type via generated client
    eprintln!("    deleting existing {language}/{dir_name}...");
    match ctx
        .client
        .androidpublisher_edits_images_deleteall(package_name, edit_id, language, api_type)
        .await
    {
        Ok(_) => {}
        Err(ref e) if format!("{e}").contains("404") => {
            // 404 means no images to delete — fine
        }
        Err(e) => {
            return Err(anyhow::anyhow!(
                "failed to delete images for {language}/{dir_name}: {e}"
            ));
        }
    }

    // Upload local images via generated client
    for file_path in &files {
        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image");
        let image_bytes = tokio::fs::read(file_path)
            .await
            .with_context(|| format!("failed to read {}", file_path.display()))?;
        let mime_type = infer_mime_type(file_path);

        match ctx
            .client
            .androidpublisher_edits_images_upload(
                package_name,
                edit_id,
                language,
                api_type,
                &image_bytes,
                mime_type,
            )
            .await
        {
            Ok(_) => eprintln!("    ✓ {filename}"),
            Err(e) => eprintln!("    ⚠ failed to upload {filename}: {e}"),
        }
    }
    Ok(())
}

pub async fn execute(args: &PushArgs, _global: &GlobalArgs) -> Result<()> {
    let ctx = GooglePlayContext::from_env().await?;
    execute_with_context(args, &ctx).await
}

/// Push catalog data using an existing Google Play context.
pub async fn execute_with_context(args: &PushArgs, ctx: &GooglePlayContext) -> Result<()> {
    let mut actions: Vec<PushAction> = Vec::new();
    let package_name = &args.package_name;

    let base_dir = args
        .input
        .as_deref()
        .map(Path::new)
        .unwrap_or_else(|| Path::new(".fastforge/stores/googleplay"))
        .join(package_name);
    eprintln!("📂 Reading sync directory: {}", base_dir.display());

    // Scan phase: enumerate local changes
    let listings_dir = base_dir.join("listings");
    for (language, _path) in listing_files(&listings_dir)? {
        actions.push(PushAction {
            resource_type: "listings".into(),
            language: Some(language.clone()),
            action: "upsert",
            details: format!("listings/{language}.yaml"),
        });
    }

    discover_screenshot_actions(&base_dir, &mut actions)?;

    let tracks_dir = base_dir.join("tracks");
    for (track_name, _path) in track_files(&tracks_dir)? {
        actions.push(PushAction {
            resource_type: "tracks".into(),
            language: None,
            action: "upsert",
            details: format!("tracks/{track_name}.yaml"),
        });
    }

    if actions.is_empty() {
        eprintln!("⚠ No changes to push. Nothing to do.");
        return Ok(());
    }

    if args.dry_run {
        eprintln!("\n📋 Dry run — would push {} actions:", actions.len());
        for a in &actions {
            eprintln!(
                "  [{:>8}] {} {} {}",
                a.action,
                a.resource_type,
                a.language.as_deref().unwrap_or(""),
                a.details
            );
        }
        return Ok(());
    }

    // Execute phase
    eprintln!("\n🚀 Creating edit...");
    let edit_id = create_edit(ctx, package_name).await?;
    eprintln!("  ✓ Edit created: {edit_id}");

    // Push listings (generated client)
    for (language, path) in listing_files(&listings_dir)? {
        let mut listing: Listing = read_yaml(&path)?;
        listing.language = Some(language.clone());

        ctx.client
            .androidpublisher_edits_listings_update(
                package_name,
                &edit_id,
                &language,
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
                &listing,
            )
            .await
            .map_err(|e| anyhow::anyhow!("failed to upsert listing '{language}': {e}"))?;
        eprintln!("  ✓ listings/{language}.yaml");
    }

    // Push screenshots & images (generated client)
    let screenshots_dir = base_dir.join("screenshots");
    if screenshots_dir.exists() {
        eprintln!("🖼️  Pushing screenshots and images...");
        for lang_entry in std::fs::read_dir(&screenshots_dir).context("reading screenshots")? {
            let lang_entry = lang_entry?;
            let lang_path = lang_entry.path();
            if !lang_path.is_dir() {
                continue;
            }
            let language = lang_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            if language.is_empty() {
                continue;
            }

            for (dir_name, api_type) in IMAGE_TYPES {
                let type_dir = lang_path.join(dir_name);
                let files = screenshot_files(&type_dir)?;
                if files.is_empty() {
                    continue;
                }
                eprintln!("  pushing {language}/{dir_name} ({} files)...", files.len());
                push_images(
                    ctx,
                    package_name,
                    &edit_id,
                    &base_dir,
                    &language,
                    dir_name,
                    api_type,
                )
                .await?;
            }
        }
    }

    // Push tracks (generated client)
    for (track_name, path) in track_files(&tracks_dir)? {
        let track: Track = read_yaml(&path)?;

        // The API's track field uses the original name including colons (e.g., "wear:production").
        // The YAML body already contains the `track` field with the correct name.
        let actual_track = track.track.as_deref().unwrap_or(&track_name).to_string();

        ctx.client
            .androidpublisher_edits_tracks_update(
                package_name,
                &edit_id,
                &actual_track,
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
                &track,
            )
            .await
            .map_err(|e| anyhow::anyhow!("failed to update track '{actual_track}': {e}"))?;
        eprintln!("  ✓ tracks/{track_name}.yaml (track: {actual_track})");
    }

    // Commit the edit
    eprintln!("  💾 Committing edit...");
    commit_edit(ctx, package_name, &edit_id).await?;
    eprintln!("  ✓ Edit committed: {edit_id}");

    eprintln!("\n✅ Push complete. {} resources pushed.", actions.len());
    Ok(())
}
