use crate::cli::GlobalArgs;
use crate::cli::commands::edit::{create_edit, delete_edit};
use crate::{GooglePlayContext, print_json, print_table};
use anyhow::Result;
use clap::{Args, Subcommand};
use serde::Serialize;

const PLAY_CONSOLE_URL_PREFIX: &str = "https://play.google.com/console/u/0/developers";

#[derive(Args, Debug)]
pub struct AppArgs {
    #[command(subcommand)]
    pub command: AppCommand,
}

#[derive(Subcommand, Debug)]
pub enum AppCommand {
    View(AppPackageArgs),
    Check(AppPackageArgs),
}

#[derive(Args, Debug)]
pub struct AppPackageArgs {
    #[arg(value_name = "PACKAGE_NAME")]
    pub package_name: String,
}

#[derive(Serialize)]
struct AppRow {
    #[serde(rename = "packageName")]
    package_name: String,
    url: String,
    checked: bool,
}

pub async fn execute(args: &AppArgs, global: &GlobalArgs) -> Result<()> {
    match &args.command {
        AppCommand::View(args) => print_app(app_row(&args.package_name, false), global),
        AppCommand::Check(args) => check(args, global).await,
    }
}

async fn check(args: &AppPackageArgs, global: &GlobalArgs) -> Result<()> {
    let ctx = GooglePlayContext::from_env().await?;
    let edit_id = create_edit(&ctx, &args.package_name).await?;
    delete_edit(&ctx, &args.package_name, &edit_id).await?;
    print_app(app_row(&args.package_name, true), global)
}

fn app_row(package_name: &str, checked: bool) -> AppRow {
    AppRow {
        package_name: package_name.to_string(),
        url: format!("{PLAY_CONSOLE_URL_PREFIX}/?app={package_name}"),
        checked,
    }
}

fn print_app(row: AppRow, global: &GlobalArgs) -> Result<()> {
    if global.json.is_some() {
        return print_json(&row, global.json.as_deref());
    }
    print_table(
        &["PACKAGE_NAME", "URL", "CHECKED"],
        &[vec![row.package_name, row.url, row.checked.to_string()]],
    );
    Ok(())
}
