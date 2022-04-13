use anyhow::Error;
use clap::{Args, Parser, Subcommand};

use super::*;

/// The default path for service config.
const DEFAULT_CONFIG_PATH: &str = "/etc/lemurs.conf";

/// The default path for the systemd service.
const DEFAULT_SERVICE_PATH: &str = "/usr/lib/systemd/system/lemurs.service";

#[derive(Parser)]
#[clap(version, about)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install default config and systemd service files
    Install(Install),
    /// Start the daemon
    Start(Start),
    /// Remove config and systemd service files
    Uninstall(Install),
}

#[derive(Args)]
struct Install {
    /// Path to the config file installation location
    #[clap(long, short)]
    #[clap(default_value_t = String::from(DEFAULT_CONFIG_PATH))]
    config: String,

    /// Path to the systemd service file installation location
    #[clap(long, short)]
    #[clap(default_value_t = String::from(DEFAULT_SERVICE_PATH))]
    service: String,
}

#[derive(Args)]
struct Start {
    /// Path to the config file installation location
    #[clap(long, short)]
    #[clap(default_value_t = String::from(DEFAULT_CONFIG_PATH))]
    config: String,
}

impl Cli {
    pub fn run(self) -> Result<(), Error> {
        match self.command {
            Commands::Install(c) => install::run(&c.config, &c.service),
            Commands::Start(c) => start::run(&c.config),
            Commands::Uninstall(c) => uninstall::run(&c.config, &c.service),
        }
    }
}
