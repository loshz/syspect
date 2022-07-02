use std::process;

use clap::Parser;
use nix::unistd::Uid;

mod bpf;
mod cli;
mod config;
mod helpers;

use crate::cli::Cli;

/// Package name loaded from Cargo.toml
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
/// Package version loaded from Cargo.toml
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // Check if running as root.
    if !Uid::effective().is_root() {
        eprintln!("error: root permission required");
        process::exit(1);
    }

    // Run the service.
    let cli = Cli::parse();
    process::exit(match cli.run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
