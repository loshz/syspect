use anyhow::Error;
use clap::{app_from_crate, App, AppSettings};

use crate::{install, start};

pub trait Command {
    fn app(&self) -> App<'static>;
    fn run(&self) -> Result<(), Error>;
}

pub struct Cli {
    install: Box<dyn Command>,
    start: Box<dyn Command>,
}

impl Cli {
    pub fn new() -> Cli {
        let install = Box::new(install::Command::new());
        let start = Box::new(start::Command::new());
        Cli { install, start }
    }

    pub fn run(self) -> Result<(), Error> {
        let app = app_from_crate!()
            .global_setting(AppSettings::PropagateVersion)
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(self.install.app())
            .subcommand(self.start.app());

        match app.get_matches().subcommand_name() {
            Some("install") => self.install.run(),
            Some("start") => self.start.run(),
            _ => Err(Error::msg("exhausted list of subcommands".to_owned())),
        }
    }
}
