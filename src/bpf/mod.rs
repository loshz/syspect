use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

use prometheus_client::encoding::EncodeLabelSet;

mod sys_enter;
use sys_enter::SysEnter;

#[derive(Clone, Hash, PartialEq, Eq, EncodeLabelSet)]
pub(crate) struct DefaultLabels {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub pname: String,
}

pub(crate) trait Program: Send + Sync {
    fn run(&self, stop: Arc<AtomicBool>, interval: Duration);
}

pub fn parse_event_programs(events: Vec<String>) -> Vec<Box<dyn Program>> {
    let mut programs: Vec<Box<dyn Program>> = Vec::new();
    for event in events {
        match event.as_str() {
            "sys_enter" => programs.push(Box::new(SysEnter)),
            &_ => eprintln!("skipping unsupported event type: `{event}`"),
        }
    }
    programs
}
