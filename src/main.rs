// src/main.rs
/*
 * Main executable for WalletTestnetHubProtocolPro
 */

use clap::Parser;
use wallettestnethubprotocolpro::{Result, run};

#[derive(Parser)]
#[command(version, about = "WalletTestnetHubProtocolPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
