use std::{thread, time};

use anyhow::Error;
use clap::ArgMatches;

use crate::{config, metrics};

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

    // Expose the Prometheus metrics.
    if let Err(_e) = metrics::serve(c.metrics_address().as_str()) {
        return Err(Error::msg("error creating metrics server"));
    };

    // simulate work
    let ten = time::Duration::from_secs(600);
    thread::sleep(ten);

    Ok(())
}
