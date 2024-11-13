use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use prometheus_client::collector::Collector;

use crate::{Error, ProgramError};

mod syscalls;

/// Represents a runnable BPF program that omits metrics.
pub trait Programmable: Send + Sync {
    /// Returns a new [`Programmable`] object.
    fn new() -> Self;

    /// Run the underlying program.
    fn run(&self) -> Result<(), ProgramError>;

    /// Returns all of the collectable metrics omitted by a program.
    fn metrics(&self) -> Vec<Box<dyn Collector>>;
}

pub struct Program<P: Programmable> {
    inner: P,
}

impl<P: Programmable> Program<P> {
    pub fn new() -> Program<P> {
        Self { inner: P::new() }
    }

    pub fn run(&self, interval: Duration, stop: Arc<AtomicBool>) {
        while !stop.load(Ordering::SeqCst) {
            if let Err(e) = self.inner.run() {
                eprintln!("{}", Error::from(e));
                break;
            }

            thread::sleep(interval);
        }
    }

    pub fn metrics(&self) -> Vec<Box<dyn Collector>> {
        self.inner.metrics()
    }
}

pub fn parse_program(s: String) -> Result<Program<impl Programmable>, ProgramError> {
    match s.as_str() {
        "sys_enter" => Ok(Program::<syscalls::SysEnter>::new()),
        &_ => Err(ProgramError::Unsupported(s)),
    }
}
