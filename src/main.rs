use clap::Parser;
use log::LevelFilter;
use nix::unistd::Uid;
use simplelog::{ColorChoice, Config, ConfigBuilder, TermLogger, TerminalMode};

mod bpf;
mod cmd;
use cmd::Cli;
mod config;
mod error;
pub use error::Error;

fn main() -> Result<(), Error> {
    Cli::parse().run()
}

/// Retuns `true` if current use is root.
pub fn is_root() -> Result<(), Error> {
    Uid::current()
        .is_root()
        .then(|| {})
        .ok_or(Error::PermissionDenied)
}

/// Configure structured logging for use when running as a system service.
pub fn configure_loging(level: &str, syslog: bool) {
    // Parse given log level and default to info.
    let max_lvl = match level.to_lowercase().as_str() {
        "debug" => LevelFilter::Debug,
        "error" => LevelFilter::Error,
        "info" => LevelFilter::Info,
        _ => LevelFilter::Info,
    };

    // Configure logger dependent on syslog formatting.
    let config = if syslog {
        ConfigBuilder::new()
            .set_time_level(LevelFilter::Off)
            .build()
    } else {
        Config::default()
    };

    TermLogger::init(max_lvl, config, TerminalMode::Mixed, ColorChoice::Never).unwrap();
}
