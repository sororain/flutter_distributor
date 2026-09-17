use crate::cli::GlobalArgs;
use crate::cli::commands::app::resolve_app;
use crate::{AppStoreConnectContext, print_json, print_table};
use anyhow::{Context as _, Result, anyhow};
use clap::{Args, Subcommand};
use serde::Serialize;
use serde_json::Value;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Args, Debug)]
pub struct BuildArgs {
    #[command(subcommand)]
    pub command: BuildCommand,
}

#[derive(Subcommand, Debug)]
pub enum BuildCommand {
    List(BuildListArgs),
    View(BuildViewArgs),
    Upload(BuildUploadArgs),
    Wait(BuildWaitArgs),
}

#[derive(Args, Debug)]
pub struct BuildListArgs {
    #[arg(long = "app")]
    pub app: String,
    #[arg(long = "version")]
    pub version: Option<String>,
}

#[derive(Args, Debug)]
pub struct BuildViewArgs {
    #[arg(value_name = "BUILD_ID")]
    pub build_id: String,
}

#[derive(Args, Debug)]
pub struct BuildUploadArgs {
    #[arg(value_name = "IPA_OR_PKG")]
    pub path: String,
    #[arg(long = "app")]
    pub app: String,
    #[arg(long = "wait", default_value_t = false)]
    pub wait: bool,
}

#[derive(Args, Debug)]
pub struct BuildWaitArgs {
    #[arg(value_name = "BUILD_ID")]
    pub build_id: String,
    #[arg(long = "timeout", default_value = "30m")]
    pub timeout: String,
}

#[derive(Serialize, Clone)]
pub struct BuildRow {
    pub id: String,
    pub version: String,
    #[serde(rename = "buildNumber")]
    pub build_number: String,
    pub platform: String,
    #[serde(rename = "processingState")]
    pub processing_state: String,
    #[serde(rename = "uploadedDate")]
    pub uploaded_date: String,
}

pub async fn execute(args: &BuildArgs, global: &GlobalArgs) -> Result<()> {
    let ctx = AppStoreConnectContext::from_env()?;
    match &args.command {
        BuildCommand::List(args) => list(&ctx, args, global).await,
        BuildCommand::View(args) => view(&ctx, args, global).await,
        BuildCommand::Upload(args) => upload(&ctx, args, global).await,
        BuildCommand::Wait(args) => wait_command(&ctx, args, global).await,
    }
}

async fn list(
    ctx: &AppStoreConnectContext,
    args: &BuildListArgs,
    global: &GlobalArgs,
) -> Result<()> {
    let app = resolve_app(ctx, &args.app).await?;
    let builds = list_builds(ctx, &app.id, args.version.as_deref(), global.limit).await?;
    print_builds(builds, global)
}

async fn view(
    ctx: &AppStoreConnectContext,
    args: &BuildViewArgs,
    global: &GlobalArgs,
) -> Result<()> {
    let build = get_build(ctx, &args.build_id).await?;
    print_builds(vec![build], global)
}

async fn upload(
    ctx: &AppStoreConnectContext,
    args: &BuildUploadArgs,
    global: &GlobalArgs,
) -> Result<()> {
    let artifact = Path::new(&args.path);
    if !artifact.exists() {
        return Err(anyhow!("artifact does not exist: {}", args.path));
    }
    let app = resolve_app(ctx, &args.app).await?;
    upload_with_altool(ctx, artifact)?;
    println!("Uploaded {} for {}", artifact.display(), app.bundle_id);

    if args.wait {
        let latest = latest_build(ctx, &app.id, None).await?;
        let timeout = parse_duration("30m")?;
        let build = wait_for_build(ctx, &latest.id, timeout).await?;
        return print_builds(vec![build], global);
    }

    Ok(())
}

async fn wait_command(
    ctx: &AppStoreConnectContext,
    args: &BuildWaitArgs,
    global: &GlobalArgs,
) -> Result<()> {
    let timeout = parse_duration(&args.timeout)?;
    let build = wait_for_build(ctx, &args.build_id, timeout).await?;
    print_builds(vec![build], global)
}

pub async fn list_builds(
    ctx: &AppStoreConnectContext,
    app_id: &str,
    version: Option<&str>,
    limit: Option<i64>,
) -> Result<Vec<BuildRow>> {
    let mut request = ctx
        .http
        .get(ctx.url("/v1/builds"))
        .query(&[("filter[app]", app_id)])
        .query(&[("sort", "-uploadedDate")]);
    if let Some(version) = version {
        request = request.query(&[("filter[preReleaseVersion.version]", version)]);
    }
    if let Some(limit) = limit {
        request = request.query(&[("limit", limit)]);
    }
    let response: Value = request.send().await?.error_for_status()?.json().await?;
    response
        .get("data")
        .and_then(Value::as_array)
        .unwrap_or(&Vec::new())
        .iter()
        .map(build_row)
        .collect()
}

pub async fn latest_build(
    ctx: &AppStoreConnectContext,
    app_id: &str,
    version: Option<&str>,
) -> Result<BuildRow> {
    list_builds(ctx, app_id, version, Some(1))
        .await?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("no build found for app {app_id}"))
}

pub async fn get_build(ctx: &AppStoreConnectContext, build_id: &str) -> Result<BuildRow> {
    let response: Value = ctx
        .http
        .get(ctx.url(&format!("/v1/builds/{build_id}")))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    build_row(
        response
            .get("data")
            .ok_or_else(|| anyhow!("missing build data"))?,
    )
}

async fn wait_for_build(
    ctx: &AppStoreConnectContext,
    build_id: &str,
    timeout: Duration,
) -> Result<BuildRow> {
    let start = Instant::now();
    loop {
        let build = get_build(ctx, build_id).await?;
        match build.processing_state.as_str() {
            "VALID" => return Ok(build),
            "FAILED" | "INVALID" => {
                return Err(anyhow!(
                    "build {build_id} processing failed with state {}",
                    build.processing_state
                ));
            }
            _ if start.elapsed() >= timeout => {
                return Err(anyhow!(
                    "failed to wait for build {build_id}: timed out after {:?}",
                    timeout
                ));
            }
            _ => tokio::time::sleep(Duration::from_secs(30)).await,
        }
    }
}

fn upload_with_altool(ctx: &AppStoreConnectContext, artifact: &Path) -> Result<()> {
    let key_dir = Path::new(&ctx.auth.key_path)
        .parent()
        .ok_or_else(|| anyhow!("APP_STORE_CONNECT_KEY_PATH must include a parent directory"))?;
    let output = Command::new("xcrun")
        .arg("altool")
        .arg("--upload-app")
        .arg("--type")
        .arg(
            if artifact.extension().and_then(|ext| ext.to_str()) == Some("pkg") {
                "macos"
            } else {
                "ios"
            },
        )
        .arg("--file")
        .arg(artifact)
        .arg("--apiKey")
        .arg(&ctx.auth.key_id)
        .arg("--apiIssuer")
        .arg(&ctx.auth.issuer_id)
        .env("API_PRIVATE_KEYS_DIR", key_dir)
        .output()
        .context("failed to run xcrun altool")?;

    if output.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    Err(anyhow!(
        "xcrun altool upload failed\nstdout: {}\nstderr: {}",
        stdout.trim(),
        stderr.trim()
    ))
}

fn print_builds(builds: Vec<BuildRow>, global: &GlobalArgs) -> Result<()> {
    if global.json.is_some() {
        return print_json(&builds, global.json.as_deref());
    }
    let rows = builds
        .into_iter()
        .map(|build| {
            vec![
                build.id,
                build.version,
                build.build_number,
                build.platform,
                build.processing_state,
                build.uploaded_date,
            ]
        })
        .collect::<Vec<_>>();
    print_table(
        &["ID", "VERSION", "BUILD", "PLATFORM", "STATE", "UPLOADED"],
        &rows,
    );
    Ok(())
}

fn build_row(value: &Value) -> Result<BuildRow> {
    let attrs = value
        .get("attributes")
        .and_then(Value::as_object)
        .ok_or_else(|| anyhow!("missing build attributes"))?;
    Ok(BuildRow {
        id: value
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
        version: attr_string(attrs, "version"),
        build_number: attr_string(attrs, "buildNumber"),
        platform: attr_string(attrs, "platform"),
        processing_state: attr_string(attrs, "processingState"),
        uploaded_date: attr_string(attrs, "uploadedDate"),
    })
}

fn attr_string(attrs: &serde_json::Map<String, Value>, key: &str) -> String {
    attrs
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

fn parse_duration(value: &str) -> Result<Duration> {
    let value = value.trim();
    let (number, multiplier) = if let Some(number) = value.strip_suffix('m') {
        (number, 60)
    } else if let Some(number) = value.strip_suffix('s') {
        (number, 1)
    } else {
        (value, 1)
    };
    let number: u64 = number
        .parse()
        .with_context(|| format!("invalid duration: {value}"))?;
    Ok(Duration::from_secs(number * multiplier))
}
