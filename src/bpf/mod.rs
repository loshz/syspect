use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use prometheus_client::collector::Collector;

use crate::Error;

pub mod sys_enter;
use sys_enter::SysEnter;

/// Represents a runnable BPF program that omits metrics.
pub trait Programmable: Send + Sync {
    /// Returns a new [`Programmable`] object.
    fn new() -> Self;

    /// Run the underlying program.
    fn run(&self) -> Result<(), Error>;

    /// Returns all of the metrics omitted by a program that should be registered with a metrics
    /// collector.
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
                eprintln!("{e}");
                break;
            }

            thread::sleep(interval);
        }
    }

    pub fn metrics(&self) -> Vec<Box<dyn Collector>> {
        self.inner.metrics()
    }
}

pub fn parse_program(s: &str) -> Result<Program<impl Programmable>, Error> {
    match s {
        "sys_enter" => Ok(Program::<SysEnter>::new()),
        &_ => Err(Error::UnsopprtedProgram(s.into())),
    }
}
