use anyhow::Error;
use clap::{Args, Parser, Subcommand};

use super::*;

/// The default path for service config.
const DEFAULT_CONFIG_PATH: &str = "/etc/syspect.conf";

/// The default path for the systemd service.
const DEFAULT_SERVICE_PATH: &str = "/usr/lib/systemd/system/syspect.service";

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
    /// Start the daemon and expose a local metrics HTTP endpoint
    Start(Start),
    /// List currently supported eBPF probes
    Probes(Probes),
    /// Remove config and systemd service files
    Uninstall(Install),
}

#[derive(Args)]
#[clap(disable_version_flag = true)]
struct Install {
    /// Path to the config file installation location
    #[clap(long, short)]
    #[clap(value_name = "PATH")]
    #[clap(default_value_t = String::from(DEFAULT_CONFIG_PATH))]
    config: String,

    /// Path to the systemd service file installation location
    #[clap(long, short)]
    #[clap(value_name = "PATH")]
    #[clap(default_value_t = String::from(DEFAULT_SERVICE_PATH))]
    service: String,
}

#[derive(Args)]
#[clap(disable_version_flag = true)]
struct Start {
    /// Path to the config file installation location
    #[clap(long, short)]
    #[clap(value_name = "PATH")]
    #[clap(default_value_t = String::from(DEFAULT_CONFIG_PATH))]
    config: String,
}

#[derive(Args)]
#[clap(disable_version_flag = true)]
struct Probes {
    /// Whether to print the output verbosely.
    #[clap(long, short)]
    verbose: bool,
}

impl Cli {
    pub fn run(self) -> Result<(), Error> {
        match self.command {
            Commands::Install(c) => install::run(&c.config, &c.service),
            Commands::Start(c) => start::run(&c.config),
            Commands::Probes(c) => probes::run(c.verbose),
            Commands::Uninstall(c) => uninstall::run(&c.config, &c.service),
        }
    }
}
