use std::process;

use nix::unistd::Uid;

mod cli;
mod config;

use crate::cli::Cli;

/// Service data loaded from Cargo.toml
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // Check if running as root.
    if !Uid::effective().is_root() {
        eprintln!("service requires root permissions");
        process::exit(1);
    }

    let cli = Cli::new();

    // Run the service.
    process::exit(match cli.run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
