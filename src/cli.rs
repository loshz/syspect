use clap::{app_from_crate, App, AppSettings, ArgMatches, ColorChoice};

pub fn app() -> ArgMatches {
    app_from_crate!()
        .global_setting(AppSettings::PropagateVersion)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .color(ColorChoice::Never)
        .subcommand(App::new("install").about("Install"))
        .subcommand(App::new("start").about("Start"))
        .get_matches()
}
