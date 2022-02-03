use clap::{app_from_crate, App, AppSettings, ArgMatches};

use crate::{install, start};

pub fn new() -> ArgMatches {
    app_from_crate!()
        .global_setting(AppSettings::PropagateVersion)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new(install::COMMAND_NAME).about("Install default config and system files."),
        )
        .subcommand(App::new(start::COMMAND_NAME).about("Start the daemon."))
        .get_matches()
}
