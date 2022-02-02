use std::process;

use anyhow::Error;
use nix::unistd::Uid;

mod cli;

use crate::cli::app;

fn main() {
    // Check if running as root.
    if !Uid::effective().is_root() {
        eprintln!("service requires root permissions");
        process::exit(1);
    }

    match app().subcommand() {
        Some(("install", _)) => println!("install"),
        Some(("start", _)) => println!("run"),
        _ => unreachable!("exhausted list of subcommands"),
    }

    // Run the service.
    process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn run() -> Result<(), Error> {
    Ok(())
}
