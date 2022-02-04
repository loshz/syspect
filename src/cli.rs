use anyhow::Error;
use clap::{app_from_crate, App, AppSettings};

use crate::{install, start};

pub struct Cli {
    app: App<'static>,
}

impl Cli {
    pub fn new() -> Cli {
        // Create cli app from crate info.
        let app = app_from_crate!()
            .global_setting(AppSettings::PropagateVersion)
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(
                App::new(install::COMMAND_NAME).about("Install default config and system files."),
            )
            .subcommand(App::new(start::COMMAND_NAME).about("Start the daemon."));

        Cli { app }
    }

    pub fn run(self) -> Result<(), Error> {
        match self.app.get_matches().subcommand() {
            Some((install::COMMAND_NAME, args)) => install::run(args),
            Some((start::COMMAND_NAME, args)) => start::run(args),
            _ => Err(Error::msg("exhausted list of subcommands".to_owned())),
        }
    }
}
