use anyhow::Error;
use clap::ArgMatches;

use crate::config;

/// The command name.
pub const COMMAND_NAME: &str = "start";

pub fn run(args: &ArgMatches) -> Result<(), Error> {
    println!(
        "Starting service, version: {} {}",
        crate::PKG_NAME,
        crate::PKG_VERSION
    );

    let path = match args.value_of("config") {
        Some(path) => path,
        _ => return Err(Error::msg("config not specified".to_owned())),
    };
    println!("Using config: {}", path);

    let c = config::from_file(path)?;

    println!(
        "Connecting to Prometheus: {}:{}",
        c.prometheus.address, c.prometheus.port
    );

    Ok(())
}
