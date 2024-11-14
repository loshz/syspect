use std::str::FromStr;
use std::sync::{atomic::AtomicBool, Arc};
use std::time::Duration;

use prometheus_client::collector::Collector;

use crate::ProgramError;

mod syscalls;

#[macro_export]
macro_rules! include_bpf {
    ($file:expr $(,)?) => {
        include!(concat!(env!("OUT_DIR"), "/", $file, ".bpf.rs"));
    };
}

/// Represents a runnable BPF program that omits metrics.
pub trait Program: Send + Sync {
    /// Returns a new [`Program`] object.
    fn new() -> Self
    where
        Self: Sized;

    /// Run the underlying program.
    fn run(&self, interval: Duration, stop: Arc<AtomicBool>) -> Result<(), ProgramError>;

    /// Returns all of the collectable metrics omitted by a program.
    fn metrics(&self) -> Vec<Box<dyn Collector>>;
}

impl FromStr for Box<dyn Program> {
    type Err = ProgramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            syscalls::SYS_ENTER => Ok(Box::new(syscalls::SysEnter::new())),
            &_ => Err(ProgramError::Unsupported(s.into())),
        }
    }
}
