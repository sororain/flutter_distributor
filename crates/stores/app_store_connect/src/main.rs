use anyhow::Result;
use clap::{Parser, Subcommand};
use fastforge_app_store_connect::cli::{
    self, ApiArgs, AppArgs, BuildArgs, GlobalArgs, SubmissionArgs, VersionArgs,
};

#[derive(Parser)]
#[command(name = "fastforge_app_store_connect")]
#[command(about = "App Store Connect command line tool.")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(flatten)]
    global: GlobalArgs,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::App(args) => cli::commands::app::execute(args, &cli.global).await?,
        Commands::Build(args) => cli::commands::build::execute(args, &cli.global).await?,
        Commands::Version(args) => cli::commands::version::execute(args, &cli.global).await?,
        Commands::Submission(args) => cli::commands::submission::execute(args, &cli.global).await?,
        Commands::Api(args) => cli::commands::api::execute(args, &cli.global).await?,
    }

    Ok(())
}
