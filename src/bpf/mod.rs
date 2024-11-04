use std::str::FromStr;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use crate::Error;

pub mod sys_enter;
use sys_enter::SysEnter;

pub(crate) trait RunnableProgram: Send + Sync {
    fn run(&self) -> Result<(), Error>;
}

pub fn parse_event_programs(events: Vec<String>) -> Vec<Program> {
    events
        .iter()
        .filter_map(|event| match event.as_str() {
            "sys_enter" => Some(Program::SysEnter),
            &_ => {
                eprintln!("warning: skipping unsupported event type: `{event}`");
                None
            }
        })
        .collect()
}

pub enum Program {
    SysEnter,
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sys_enter" => Ok(Self::SysEnter),
            &_ => Err(Error::UnsopprtedProgram(s.into())),
        }
    }
}

impl Program {
    pub fn run(&self, stop: Arc<AtomicBool>, interval: Duration) {
        let program = match self {
            Self::SysEnter => SysEnter::new(),
        };

        while !stop.load(Ordering::SeqCst) {
            if let Err(_e) = program.run() {
                // TODO: print error.
                break;
            }

            thread::sleep(interval);
        }
    }
}
