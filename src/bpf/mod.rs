use std::str::FromStr;
use std::sync::{atomic::AtomicBool, Arc};
use std::time::Duration;

use prometheus_client::collector::Collector;

use crate::{config::Tracing, ProgramError};

pub(crate) mod ffi;
mod raw_syscalls;

/// List of curentlt support Kernel tracing events found in `/sys/kernel/debug/tracing/events/`
pub const SUPPORTED_EVENTS: &[&str] = &["raw_syscalls:sys_enter"];

/// Represents a runnable BPF program that omits metrics.
pub trait Program: Send + Sync {
    /// Returns a new [`Program`] object.
    fn new() -> Self
    where
        Self: Sized;

    /// Run the underlying program.
    fn run(&self, opts: ProgramOptions, stop: Arc<AtomicBool>) -> Result<(), ProgramError>;

    /// Returns all of the collectable metrics omitted by a program.
    fn metrics(&self) -> Vec<Box<dyn Collector>>;
}

impl FromStr for Box<dyn Program> {
    type Err = ProgramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            raw_syscalls::SYS_ENTER => Ok(Box::new(raw_syscalls::SysEnter::new())),
            &_ => Err(ProgramError::Unsupported(s.into())),
        }
    }
}

/// Runtime options passed to a program.
pub struct ProgramOptions {
    pub debug: bool,
    pub interval: Duration,
}

impl From<&Tracing> for ProgramOptions {
    fn from(config: &Tracing) -> Self {
        Self {
            debug: config.debug,
            interval: config.interval,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::config::Tracing;

    use super::ProgramOptions;

    #[test]
    fn test_programoptions_from_config() {
        let config = Tracing {
            debug: true,
            interval: Duration::from_secs(10),
            raw_syscalls: vec![],
        };
        let progam_options = ProgramOptions::from(&config);
        assert!(progam_options.debug);
        assert_eq!(Duration::from_secs(10), progam_options.interval);
    }
}
