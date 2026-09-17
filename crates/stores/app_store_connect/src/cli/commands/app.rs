use crate::cli::GlobalArgs;
use crate::{AppStoreConnectContext, print_json, print_table};
use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use serde::Serialize;
use serde_json::Value;

#[derive(Args, Debug)]
pub struct AppArgs {
    #[command(subcommand)]
    pub command: AppCommand,
}

#[derive(Subcommand, Debug)]
pub enum AppCommand {
    List,
    View(AppViewArgs),
}

#[derive(Args, Debug)]
pub struct AppViewArgs {
    #[arg(value_name = "APP")]
    pub app: String,
}

#[derive(Serialize)]
pub struct AppRow {
    pub id: String,
    pub name: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    pub sku: String,
    pub locale: String,
}

pub async fn execute(args: &AppArgs, global: &GlobalArgs) -> Result<()> {
    let ctx = AppStoreConnectContext::from_env()?;
    match &args.command {
        AppCommand::List => list(&ctx, global).await,
        AppCommand::View(args) => view(&ctx, args, global).await,
    }
}

async fn list(ctx: &AppStoreConnectContext, global: &GlobalArgs) -> Result<()> {
    let mut request = ctx.http.get(ctx.url("/v1/apps"));
    if let Some(limit) = global.limit {
        request = request.query(&[("limit", limit)]);
    }
    let response: Value = request.send().await?.error_for_status()?.json().await?;
    let apps = app_rows(
        response
            .get("data")
            .and_then(Value::as_array)
            .unwrap_or(&vec![]),
    )?;
    print_apps(apps, global)
}

async fn view(ctx: &AppStoreConnectContext, args: &AppViewArgs, global: &GlobalArgs) -> Result<()> {
    let app = resolve_app(ctx, &args.app).await?;
    print_apps(vec![app], global)
}

pub async fn resolve_app(ctx: &AppStoreConnectContext, app: &str) -> Result<AppRow> {
    if app.chars().all(|c| c.is_ascii_digit()) {
        let response = ctx
            .http
            .get(ctx.url(&format!("/v1/apps/{app}")))
            .send()
            .await?;
        if response.status().is_success() {
            let value: Value = response.json().await?;
            return app_row(
                value
                    .get("data")
                    .ok_or_else(|| anyhow!("missing app data"))?,
            );
        }
    }

    for filter in ["filter[bundleId]", "filter[sku]", "filter[name]"] {
        let response: Value = ctx
            .http
            .get(ctx.url("/v1/apps"))
            .query(&[(filter, app), ("limit", "2")])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let data = response
            .get("data")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("missing apps data"))?;
        if data.len() == 1 {
            return app_row(&data[0]);
        }
        if data.len() > 1 {
            return Err(anyhow!("app `{app}` matched multiple records"));
        }
    }

    Err(anyhow!("app `{app}` not found"))
}

fn print_apps(apps: Vec<AppRow>, global: &GlobalArgs) -> Result<()> {
    if global.json.is_some() {
        return print_json(&apps, global.json.as_deref());
    }
    let rows = apps
        .into_iter()
        .map(|app| vec![app.id, app.name, app.bundle_id, app.sku, app.locale])
        .collect::<Vec<_>>();
    print_table(&["ID", "NAME", "BUNDLE_ID", "SKU", "LOCALE"], &rows);
    Ok(())
}

fn app_rows(values: &[Value]) -> Result<Vec<AppRow>> {
    values.iter().map(app_row).collect()
}

fn app_row(value: &Value) -> Result<AppRow> {
    let attrs = value
        .get("attributes")
        .and_then(Value::as_object)
        .ok_or_else(|| anyhow!("missing app attributes"))?;
    Ok(AppRow {
        id: value
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
        name: attr_string(attrs, "name"),
        bundle_id: attr_string(attrs, "bundleId"),
        sku: attr_string(attrs, "sku"),
        locale: attr_string(attrs, "primaryLocale"),
    })
}

fn attr_string(attrs: &serde_json::Map<String, Value>, key: &str) -> String {
    attrs
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}
