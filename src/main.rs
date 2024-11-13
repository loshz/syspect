use std::process::ExitCode;

use clap::Parser;

mod bpf;
mod cmd;
use cmd::Cli;
mod config;
mod error;
pub use error::*;
mod metrics;

fn main() -> ExitCode {
    if let Err(e) = Cli::parse().run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
