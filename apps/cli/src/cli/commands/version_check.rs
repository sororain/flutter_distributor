use anyhow::Result;
use clap::Args;

/// Check whether a newer version of fastforge is available on crates.io.
#[derive(Args)]
pub struct VersionCheckArgs {
    /// Print the current version and exit without checking for updates.
    #[arg(long = "current-only", default_value_t = false)]
    pub current_only: bool,
}

pub async fn execute(_args: &VersionCheckArgs) -> Result<()> {
    let current = env!("CARGO_PKG_VERSION");
    println!("fastforge {current}");
    // TODO(M5): fetch latest version from crates.io and compare.
    Ok(())
}
