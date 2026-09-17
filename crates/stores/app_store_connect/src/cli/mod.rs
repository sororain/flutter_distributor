pub mod commands;

use anyhow::Result;
use clap::{Args, Subcommand};

pub use commands::api::ApiArgs;
pub use commands::app::AppArgs;
pub use commands::build::BuildArgs;
pub use commands::catalog::CatalogArgs;
pub use commands::submission::SubmissionArgs;
pub use commands::version::VersionArgs;

#[derive(Args, Debug)]
pub struct AppStoreConnectArgs {
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: AppStoreConnectCommand,
}

#[derive(Subcommand, Debug)]
pub enum AppStoreConnectCommand {
    #[command(about = "Query App Store Connect apps")]
    App(AppArgs),
    #[command(about = "Query, upload, and wait for builds")]
    Build(BuildArgs),
    #[command(about = "Query and submit App Store versions")]
    Version(VersionArgs),
    #[command(about = "Manage App Store review submissions")]
    Submission(SubmissionArgs),
    #[command(about = "Call raw App Store Connect API endpoints")]
    Api(ApiArgs),
    #[command(about = "Manage your app's store data (metadata, screenshots, previews)")]
    Catalog(CatalogArgs),
}

pub async fn execute(root: &AppStoreConnectArgs) -> Result<()> {
    match &root.command {
        AppStoreConnectCommand::App(args) => commands::app::execute(args, &root.global).await,
        AppStoreConnectCommand::Build(args) => commands::build::execute(args, &root.global).await,
        AppStoreConnectCommand::Version(args) => {
            commands::version::execute(args, &root.global).await
        }
        AppStoreConnectCommand::Submission(args) => {
            commands::submission::execute(args, &root.global).await
        }
        AppStoreConnectCommand::Api(args) => commands::api::execute(args, &root.global).await,
        AppStoreConnectCommand::Catalog(args) => {
            commands::catalog::execute(args, &root.global).await
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct GlobalArgs {
    #[arg(long = "json", value_name = "FIELDS", global = true)]
    pub json: Option<String>,
    #[arg(long = "limit", global = true)]
    pub limit: Option<i64>,
    #[arg(long = "paginate", default_value_t = false, global = true)]
    pub paginate: bool,
    #[arg(long = "verbose", default_value_t = false, global = true)]
    pub verbose: bool,
    #[arg(long = "debug", default_value_t = false, global = true)]
    pub debug: bool,
    #[arg(long = "no-color", default_value_t = false, global = true)]
    pub no_color: bool,
}
