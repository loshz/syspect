use std::process;

use nix::unistd::Uid;

mod cli;
mod install;
mod start;
mod uninstall;

use crate::cli::*;

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
