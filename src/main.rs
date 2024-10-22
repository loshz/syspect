use std::process::ExitCode;

use clap::Parser;
use nix::unistd::Uid;

mod bpf;
mod cmd;
use cmd::Cli;
mod config;
mod error;
pub use error::Error;
mod metrics;

fn main() -> ExitCode {
    if let Err(e) = Cli::parse().run() {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

/// Retuns `true` if current use is root.
pub fn is_root() -> Result<(), Error> {
    Uid::current()
        .is_root()
        .then_some(())
        .ok_or(Error::PermissionDenied)
}
