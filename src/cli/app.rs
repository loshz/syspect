use anyhow::Error;
use clap::{Args, Parser, Subcommand};

use super::*;

/// The default path for service config.
const DEFAULT_CONFIG_PATH: &str = "/etc/syspect.conf";

/// The default path for the systemd service.
const DEFAULT_SERVICE_PATH: &str = "/usr/lib/systemd/system/syspect.service";

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install default config and systemd service files
    Install(Install),
    /// Start the daemon and expose a local metrics HTTP endpoint
    Start(Start),
    /// List currently available Kernel trace events
    Events(Events),
    /// Remove config and systemd service files
    Uninstall(Install),
}

#[derive(Args)]
#[command(disable_version_flag = true)]
struct Install {
    /// Path to the config file installation location
    #[arg(long, short)]
    #[arg(value_name = "PATH")]
    #[arg(default_value_t = String::from(DEFAULT_CONFIG_PATH))]
    config: String,

    /// Path to the systemd service file installation location
    #[arg(long, short)]
    #[arg(value_name = "PATH")]
    #[arg(default_value_t = String::from(DEFAULT_SERVICE_PATH))]
    service: String,
}

#[derive(Args)]
#[command(disable_version_flag = true)]
struct Start {
    /// Path to the config file installation location
    #[arg(long, short)]
    #[arg(value_name = "PATH")]
    #[arg(default_value_t = String::from(DEFAULT_CONFIG_PATH))]
    config: String,
}

#[derive(Args)]
#[command(disable_version_flag = true)]
struct Events {
    /// Whether to print the output verbosely.
    #[arg(long, short)]
    verbose: bool,
}

impl Cli {
    pub fn run(self) -> Result<(), Error> {
        match self.command {
            Commands::Install(c) => install::run(&c.config, &c.service),
            Commands::Start(c) => start::run(&c.config),
            Commands::Events(c) => events::run(c.verbose),
            Commands::Uninstall(c) => uninstall::run(&c.config, &c.service),
        }
    }
}
