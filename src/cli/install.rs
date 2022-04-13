use std::fs::File;
use std::io::prelude::*;

use anyhow::{Context, Error};

/// An example config file with defaults loaded at compile time.
const DEFAULT_CONFIG: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config/lemurs.conf"));

/// A preconfigured systemd service file loaded at compile time.
const DEFAULT_SERVICE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/config/lemurs.service"
));

pub fn run(config: &str, service: &str) -> Result<(), Error> {
    // Write config file.
    write_file(config, DEFAULT_CONFIG).context("error writing config file")?;
    println!("Default config saved to: {}", config);
    //
    // Write systemd service file.
    write_file(service, DEFAULT_SERVICE).context("error writing systemd service file")?;
    println!("systemd service saved to: {}", service);

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
