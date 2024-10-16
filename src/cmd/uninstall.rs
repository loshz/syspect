use std::fs;

use crate::Error;

pub fn run(config: &str, service: &str) -> Result<(), Error> {
    // Remove config file.
    fs::remove_file(config).map_err(Error::IO)?;
    println!("Removed config file: {config}");

    // Remove systemd service file.
    fs::remove_file(service).map_err(Error::IO)?;
    println!("Removed systemd service file: {service}");

    Ok(())
}
