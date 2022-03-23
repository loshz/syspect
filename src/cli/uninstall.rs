use std::fs;

use anyhow::{Context, Error};
use clap::ArgMatches;

/// The command name.
pub const COMMAND_NAME: &str = "uninstall";

pub fn run(args: &ArgMatches) -> Result<(), Error> {
    // Remove config file.
    if let Some(path) = args.value_of("config") {
        fs::remove_file(path).context("error removing config file")?;
        println!("Removed config file: {}", path);
    }

    // Remove systemd service file.
    if let Some(path) = args.value_of("service") {
        fs::remove_file(path).context("error removing systemd service file")?;
        println!("Removed systemd service file: {}", path);
    }

    Ok(())
}
