use anyhow::Error;
use clap::App;

/// The command name.
const COMMAND_NAME: &str = "install";

pub struct Command;

impl Command {
    pub fn new() -> Command {
        Command {}
    }
}

impl crate::cli::Command for Command {
    fn app(&self) -> App<'static> {
        App::new(COMMAND_NAME).about("Start the daemon.")
    }
    fn run(&self) -> Result<(), Error> {
        println!("{}", COMMAND_NAME);
        Ok(())
    }
}
