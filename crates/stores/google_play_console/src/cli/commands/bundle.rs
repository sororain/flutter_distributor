use crate::cli::GlobalArgs;
use crate::cli::commands::edit::{commit_edit, create_edit};
use crate::cli::commands::track::update_track;
use crate::{GooglePlayContext, print_json, print_table};
use anyhow::{Context as _, Result, anyhow};
use clap::{Args, Subcommand};
use serde::Serialize;
use serde_json::Value;
use std::path::Path;
use tokio::fs;

#[derive(Args, Debug)]
pub struct BundleArgs {
    #[command(subcommand)]
    pub command: BundleCommand,
}

#[derive(Subcommand, Debug)]
pub enum BundleCommand {
    Upload(BundleUploadArgs),
}

#[derive(Args, Debug)]
pub struct BundleUploadArgs {
    #[arg(value_name = "AAB")]
    pub path: String,
    #[arg(long = "package-name")]
    pub package_name: String,
    #[arg(long = "edit-id")]
    pub edit_id: Option<String>,
    #[arg(long = "track")]
    pub track: Option<String>,
    #[arg(long = "release-name")]
    pub release_name: Option<String>,
    #[arg(long = "status", default_value = "completed")]
    pub status: String,
    #[arg(long = "commit", default_value_t = false)]
    pub commit: bool,
}

#[derive(Serialize)]
struct BundleUploadRow {
    #[serde(rename = "packageName")]
    package_name: String,
    #[serde(rename = "editId")]
    edit_id: String,
    #[serde(rename = "versionCode")]
    version_code: i64,
    track: String,
    committed: bool,
}

pub async fn execute(args: &BundleArgs, global: &GlobalArgs) -> Result<()> {
    let ctx = GooglePlayContext::from_env().await?;
    match &args.command {
        BundleCommand::Upload(args) => upload(&ctx, args, global).await,
    }
}

async fn upload(
    ctx: &GooglePlayContext,
    args: &BundleUploadArgs,
    global: &GlobalArgs,
) -> Result<()> {
    ensure_bundle(&args.path)?;
    let edit_id = match &args.edit_id {
        Some(edit_id) => edit_id.clone(),
        None => create_edit(ctx, &args.package_name).await?,
    };
    let version_code = upload_bundle(ctx, &args.package_name, &edit_id, &args.path).await?;

    if let Some(track) = &args.track {
        let release_name = args
            .release_name
            .clone()
            .unwrap_or_else(|| build_release_name(&args.path, version_code));
        update_track(
            ctx,
            &args.package_name,
            &edit_id,
            track,
            &release_name,
            version_code,
            &args.status,
        )
        .await?;
    }

    if args.commit {
        commit_edit(ctx, &args.package_name, &edit_id).await?;
    }

    let row = BundleUploadRow {
        package_name: args.package_name.clone(),
        edit_id,
        version_code,
        track: args.track.clone().unwrap_or_default(),
        committed: args.commit,
    };
    print_bundle_upload(row, global)
}

async fn upload_bundle(
    ctx: &GooglePlayContext,
    package_name: &str,
    edit_id: &str,
    path: &str,
) -> Result<i64> {
    let bytes = fs::read(path)
        .await
        .with_context(|| format!("failed to read bundle: {path}"))?;
    let response: Value = ctx
        .http
        .post(ctx.upload_url(&format!(
            "/applications/{package_name}/edits/{edit_id}/bundles?uploadType=media"
        )))
        .header("content-type", "application/octet-stream")
        .body(bytes)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    response
        .get("versionCode")
        .and_then(Value::as_i64)
        .ok_or_else(|| anyhow!("missing versionCode in bundle upload response"))
}

fn print_bundle_upload(row: BundleUploadRow, global: &GlobalArgs) -> Result<()> {
    if global.json.is_some() {
        return print_json(&row, global.json.as_deref());
    }
    print_table(
        &[
            "PACKAGE_NAME",
            "EDIT_ID",
            "VERSION_CODE",
            "TRACK",
            "COMMITTED",
        ],
        &[vec![
            row.package_name,
            row.edit_id,
            row.version_code.to_string(),
            row.track,
            row.committed.to_string(),
        ]],
    );
    Ok(())
}

fn ensure_bundle(path: &str) -> Result<()> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(anyhow!("bundle does not exist: {}", path.display()));
    }
    if path.extension().and_then(|ext| ext.to_str()) != Some("aab") {
        return Err(anyhow!("Google Play bundle upload only supports .aab"));
    }
    Ok(())
}

fn build_release_name(path: &str, version_code: i64) -> String {
    Path::new(path)
        .file_stem()
        .and_then(|name| name.to_str())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| format!("release {version_code}"))
}
