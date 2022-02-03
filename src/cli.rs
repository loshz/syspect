use clap::{app_from_crate, App, AppSettings, ArgMatches, ColorChoice};

use crate::{install, start};

pub fn new() -> ArgMatches {
    app_from_crate!()
        .global_setting(AppSettings::PropagateVersion)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::HidePossibleValues)
        .color(ColorChoice::Never)
        .subcommand(App::new(install::COMMAND_NAME).about("Install"))
        .subcommand(App::new(start::COMMAND_NAME).about("Start"))
        .get_matches()
}
