use anyhow::Error;
use clap::ArgMatches;

/// The command name.
pub const COMMAND_NAME: &str = "start";

pub fn run(_args: &ArgMatches) -> Result<(), Error> {
    println!("{}", COMMAND_NAME);
    Ok(())
}
