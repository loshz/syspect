use std::fs::File;
use std::io::prelude::*;

use anyhow::{Context, Error};
use clap::ArgMatches;

/// The command name.
pub const COMMAND_NAME: &str = "install";

/// The default path for service config.
pub const DEFAULT_CONFIG_PATH: &str = "/etc/lemurs.conf";

/// The default path for the systemd service.
pub const DEFAULT_SERVICE_PATH: &str = "/usr/lib/systemd/system/lemurs.service";

/// An example config file with defaults loaded at compile time.
const DEFAULT_CONFIG: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config/lemurs.conf"));

/// A preconfigured systemd service file loaded at compile time.
const DEFAULT_SERVICE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/config/lemurs.service"
));

pub fn run(args: &ArgMatches) -> Result<(), Error> {
    // Write config file.
    if let Some(path) = args.value_of("config") {
        write_file(path, DEFAULT_CONFIG).context("error writing config file")?;
        println!("Default config saved to: {}", path);
    }
    //
    // Write systemd service file.
    if let Some(path) = args.value_of("service") {
        write_file(path, DEFAULT_SERVICE).context("error writing systemd service file")?;
        println!("systemd service saved to: {}", path);
    }

    Ok(())
}

/// Creates a new or truncates an existing file and writes the given
/// data before attempting to sync the filesystem.
fn write_file(path: &str, data: &str) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(data.as_bytes())?;

    f.sync_all()?;
    Ok(())
}
