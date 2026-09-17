use crate::cli::GlobalArgs;
use crate::{GooglePlayContext, print_json, print_table};
use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Args, Debug)]
pub struct TrackArgs {
    #[command(subcommand)]
    pub command: TrackCommand,
}

#[derive(Subcommand, Debug)]
pub enum TrackCommand {
    List(TrackListArgs),
    View(TrackViewArgs),
    Update(TrackUpdateArgs),
}

#[derive(Args, Debug)]
pub struct TrackListArgs {
    #[arg(long = "package-name")]
    pub package_name: String,
    #[arg(long = "edit-id")]
    pub edit_id: String,
}

#[derive(Args, Debug)]
pub struct TrackViewArgs {
    #[arg(value_name = "TRACK")]
    pub track: String,
    #[arg(long = "package-name")]
    pub package_name: String,
    #[arg(long = "edit-id")]
    pub edit_id: String,
}

#[derive(Args, Debug)]
pub struct TrackUpdateArgs {
    #[arg(value_name = "TRACK")]
    pub track: String,
    #[arg(long = "package-name")]
    pub package_name: String,
    #[arg(long = "edit-id")]
    pub edit_id: String,
    #[arg(long = "version-code")]
    pub version_code: i64,
    #[arg(long = "status", default_value = "completed")]
    pub status: String,
    #[arg(long = "release-name")]
    pub release_name: Option<String>,
}

#[derive(Serialize)]
pub struct TrackRow {
    pub track: String,
    pub status: String,
    #[serde(rename = "versionCodes")]
    pub version_codes: String,
    pub name: String,
}

pub async fn execute(args: &TrackArgs, global: &GlobalArgs) -> Result<()> {
    let ctx = GooglePlayContext::from_env().await?;
    match &args.command {
        TrackCommand::List(args) => list(&ctx, args, global).await,
        TrackCommand::View(args) => view(&ctx, args, global).await,
        TrackCommand::Update(args) => update(&ctx, args, global).await,
    }
}

async fn list(ctx: &GooglePlayContext, args: &TrackListArgs, global: &GlobalArgs) -> Result<()> {
    let response: Value = ctx
        .http
        .get(ctx.api_url(&format!(
            "/applications/{}/edits/{}/tracks",
            args.package_name, args.edit_id
        )))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let tracks = response
        .get("tracks")
        .and_then(Value::as_array)
        .unwrap_or(&Vec::new())
        .iter()
        .map(track_row)
        .collect::<Result<Vec<_>>>()?;
    print_tracks(tracks, global)
}

async fn view(ctx: &GooglePlayContext, args: &TrackViewArgs, global: &GlobalArgs) -> Result<()> {
    let track = get_track(ctx, &args.package_name, &args.edit_id, &args.track).await?;
    print_tracks(vec![track], global)
}

async fn update(
    ctx: &GooglePlayContext,
    args: &TrackUpdateArgs,
    global: &GlobalArgs,
) -> Result<()> {
    let release_name = args
        .release_name
        .clone()
        .unwrap_or_else(|| format!("release {}", args.version_code));
    let track = update_track(
        ctx,
        &args.package_name,
        &args.edit_id,
        &args.track,
        &release_name,
        args.version_code,
        &args.status,
    )
    .await?;
    print_tracks(vec![track], global)
}

pub async fn update_track(
    ctx: &GooglePlayContext,
    package_name: &str,
    edit_id: &str,
    track: &str,
    release_name: &str,
    version_code: i64,
    status: &str,
) -> Result<TrackRow> {
    let body = json!({
        "track": track,
        "releases": [{
            "name": release_name,
            "versionCodes": [version_code.to_string()],
            "status": status
        }]
    });
    let response: Value = ctx
        .http
        .put(ctx.api_url(&format!(
            "/applications/{package_name}/edits/{edit_id}/tracks/{track}"
        )))
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    track_row(&response)
}

async fn get_track(
    ctx: &GooglePlayContext,
    package_name: &str,
    edit_id: &str,
    track: &str,
) -> Result<TrackRow> {
    let response: Value = ctx
        .http
        .get(ctx.api_url(&format!(
            "/applications/{package_name}/edits/{edit_id}/tracks/{track}"
        )))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    track_row(&response)
}

fn print_tracks(tracks: Vec<TrackRow>, global: &GlobalArgs) -> Result<()> {
    if global.json.is_some() {
        return print_json(&tracks, global.json.as_deref());
    }
    let rows = tracks
        .into_iter()
        .map(|track| vec![track.track, track.status, track.version_codes, track.name])
        .collect::<Vec<_>>();
    print_table(&["TRACK", "STATUS", "VERSION_CODES", "NAME"], &rows);
    Ok(())
}

fn track_row(value: &Value) -> Result<TrackRow> {
    let track = value
        .get("track")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let release = value
        .get("releases")
        .and_then(Value::as_array)
        .and_then(|releases| releases.first());
    let status = release
        .and_then(|release| release.get("status"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let name = release
        .and_then(|release| release.get("name"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let version_codes = release
        .and_then(|release| release.get("versionCodes"))
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("missing track versionCodes"))
        .map(|codes| {
            codes
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_default();
    Ok(TrackRow {
        track,
        status,
        version_codes,
        name,
    })
}
