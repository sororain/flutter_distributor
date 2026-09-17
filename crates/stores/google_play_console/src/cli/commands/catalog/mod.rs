pub mod pull;
pub mod push;

use crate::cli::GlobalArgs;
use anyhow::Result;
use clap::{Args, Subcommand};

pub use pull::PullArgs;
pub use push::PushArgs;

#[derive(Args, Debug)]
pub struct CatalogArgs {
    #[command(subcommand)]
    pub command: CatalogCommand,
}

#[derive(Subcommand, Debug)]
pub enum CatalogCommand {
    #[command(about = "Pull Google Play store data to a local directory for editing")]
    Pull(PullArgs),
    #[command(about = "Push local changes back to Google Play")]
    Push(PushArgs),
}

pub async fn execute(args: &CatalogArgs, global: &GlobalArgs) -> Result<()> {
    match &args.command {
        CatalogCommand::Pull(pull_args) => pull::execute(pull_args, global).await,
        CatalogCommand::Push(push_args) => push::execute(push_args, global).await,
    }
}
