use std::str::FromStr;

use prometheus_client::collector::Collector;

use crate::Error;

pub mod sys_enter;
use sys_enter::SysEnter;

pub trait Program: Send + Sync + Collector {
    fn run(&self) -> Result<(), Error>;
}

impl FromStr for Box<dyn Program> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sys_enter" => Ok(Box::new(SysEnter::new())),
            &_ => Err(Error::UnsopprtedProgram(s.into())),
        }
    }
}
