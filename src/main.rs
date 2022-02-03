use std::process;

use anyhow::Error;
use nix::unistd::Uid;

mod cli;
mod install;
mod start;

fn main() {
    // Check if running as root.
    if !Uid::effective().is_root() {
        eprintln!("service requires root permissions");
        process::exit(1);
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
    match cli::new().subcommand_name() {
        Some(install::COMMAND_NAME) => install::run(),
        Some(start::COMMAND_NAME) => start::run(),
        _ => Err(Error::msg("exhausted list of subcommands".to_owned())),
    }
}
