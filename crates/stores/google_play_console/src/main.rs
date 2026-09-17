use anyhow::Result;
use clap::{Parser, Subcommand};
use fastforge_google_play_console::cli::{
    self, ApiArgs, AppArgs, BundleArgs, CatalogArgs, EditArgs, GlobalArgs, TrackArgs,
};

#[derive(Parser)]
#[command(name = "fastforge_google_play_console")]
#[command(about = "Google Play Console command line tool.")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(flatten)]
    global: GlobalArgs,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::App(args) => cli::commands::app::execute(args, &cli.global).await?,
        Commands::Edit(args) => cli::commands::edit::execute(args, &cli.global).await?,
        Commands::Bundle(args) => cli::commands::bundle::execute(args, &cli.global).await?,
        Commands::Track(args) => cli::commands::track::execute(args, &cli.global).await?,
        Commands::Catalog(args) => cli::commands::catalog::execute(args, &cli.global).await?,
        Commands::Api(args) => cli::commands::api::execute(args, &cli.global).await?,
    }
    Ok(())
}
