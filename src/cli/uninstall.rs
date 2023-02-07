use std::fs;

use anyhow::{Context, Error};

pub fn run(config: &str, service: &str) -> Result<(), Error> {
    // Remove config file.
    fs::remove_file(config).context("error removing config file")?;
    println!("Removed config file: {config}");

    // Remove systemd service file.
    fs::remove_file(service).context("error removing systemd service file")?;
    println!("Removed systemd service file: {service}");

    Ok(())
}
