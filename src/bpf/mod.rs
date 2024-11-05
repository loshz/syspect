use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use crate::Error;

pub mod sys_enter;
use sys_enter::SysEnter;

pub trait Programmable: Send + Sync {
    fn new() -> Self;
    fn run(&self) -> Result<(), Error>;
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
}

pub fn parse_program(s: &str) -> Result<Program<impl Programmable>, Error> {
    match s {
        "sys_enter" => Ok(Program::<SysEnter>::new()),
        &_ => Err(Error::UnsopprtedProgram(s.into())),
    }
}
