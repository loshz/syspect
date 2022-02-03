use std::fs::File;
use std::io::prelude::*;

use anyhow::{Context, Error};

/// The command name.
pub const COMMAND_NAME: &str = "install";

/// The default path for service config.
const CONFIG_PATH: &str = "/etc/lemurs.conf";

/// An example config file with defaults.
const CONFIG_DEFAULT: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config/lemurs.conf"));

/// The default path for the systemd service.
const SERVICE_PATH: &str = "/usr/lib/systemd/system/lemurs.service";

/// A preconfigured systemd service file.
const SERVICE_DEFAULT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/config/lemurs.service"
));

pub fn run() -> Result<(), Error> {
    // Write config file.
    write_file(CONFIG_PATH, CONFIG_DEFAULT).context("error writing config file")?;
    println!("Default config saved to: {}", CONFIG_PATH);

    // Write systemd service file.
    write_file(SERVICE_PATH, SERVICE_DEFAULT).context("error writing systemd service file")?;
    println!("systemd service saved to: {}", SERVICE_PATH);

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
