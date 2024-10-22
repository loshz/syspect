use std::sync::{atomic::AtomicBool, Arc};
use std::time::Duration;

pub mod sys_enter;
use sys_enter::SysEnter;

pub(crate) trait Program: Send + Sync {
    fn run(&self, stop: Arc<AtomicBool>, interval: Duration);
}

pub fn parse_event_programs(events: Vec<String>) -> Vec<Box<dyn Program>> {
    events
        .iter()
        .filter_map(|event| match event.as_str() {
            "sys_enter" => Some(Box::new(SysEnter::new()) as _),
            &_ => {
                eprintln!("warning: skipping unsupported event type: `{event}`");
                None
            }
        })
        .collect()
}
