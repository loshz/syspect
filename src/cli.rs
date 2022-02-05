use anyhow::Error;
use clap::{app_from_crate, App, AppSettings, Arg};

use crate::{install, start};

pub struct Cli {
    app: App<'static>,
}

impl Cli {
    pub fn new() -> Cli {
        // Create subcommands
        let install = App::new(install::COMMAND_NAME)
            .setting(AppSettings::DisableVersionFlag)
            .about("Install default config and systemd service files.")
            .arg(
                Arg::new("config")
                    .long("conf")
                    .takes_value(true)
                    .value_name("PATH")
                    .help("Path to the config file installation location")
                    .default_value(install::DEFAULT_CONFIG_PATH),
            )
            .arg(
                Arg::new("service")
                    .long("service")
                    .takes_value(true)
                    .value_name("PATH")
                    .help("Path to the systemd service file installation location")
                    .default_value(install::DEFAULT_SERVICE_PATH),
            );
        let start = App::new(start::COMMAND_NAME).about("Start the daemon.");

        // Create cli app from crate info.
        let app = app_from_crate!()
            .global_setting(AppSettings::PropagateVersion)
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(install)
            .subcommand(start);

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
