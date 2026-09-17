pub mod commands;

use anyhow::Result;
use clap::{Args, Subcommand};

pub use commands::api::ApiArgs;
pub use commands::app::AppArgs;
pub use commands::bundle::BundleArgs;
pub use commands::catalog::{CatalogArgs, CatalogCommand};
pub use commands::edit::EditArgs;
pub use commands::track::TrackArgs;

#[derive(Args, Debug)]
pub struct GooglePlayConsoleArgs {
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: GooglePlayConsoleCommand,
}

#[derive(Subcommand, Debug)]
pub enum GooglePlayConsoleCommand {
    #[command(about = "Show and validate Google Play apps")]
    App(AppArgs),
    #[command(about = "Create and commit Google Play edits")]
    Edit(EditArgs),
    #[command(about = "Upload Android App Bundles")]
    Bundle(BundleArgs),
    #[command(about = "List and update tracks")]
    Track(TrackArgs),
    #[command(about = "Manage your app's store listing and track data (metadata)")]
    Catalog(CatalogArgs),
    #[command(about = "Call raw Google Play Developer API endpoints")]
    Api(ApiArgs),
}

pub async fn execute(root: &GooglePlayConsoleArgs) -> Result<()> {
    match &root.command {
        GooglePlayConsoleCommand::App(args) => commands::app::execute(args, &root.global).await,
        GooglePlayConsoleCommand::Edit(args) => commands::edit::execute(args, &root.global).await,
        GooglePlayConsoleCommand::Bundle(args) => {
            commands::bundle::execute(args, &root.global).await
        }
        GooglePlayConsoleCommand::Track(args) => commands::track::execute(args, &root.global).await,
        GooglePlayConsoleCommand::Catalog(args) => {
            commands::catalog::execute(args, &root.global).await
        }
        GooglePlayConsoleCommand::Api(args) => commands::api::execute(args, &root.global).await,
    }
}

#[derive(Args, Debug, Clone)]
pub struct GlobalArgs {
    #[arg(long = "json", value_name = "FIELDS", global = true)]
    pub json: Option<String>,
    #[arg(long = "limit", global = true)]
    pub limit: Option<i64>,
    #[arg(long = "verbose", default_value_t = false, global = true)]
    pub verbose: bool,
    #[arg(long = "debug", default_value_t = false, global = true)]
    pub debug: bool,
    #[arg(long = "no-color", default_value_t = false, global = true)]
    pub no_color: bool,
}
