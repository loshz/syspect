use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::bpf::Program;

pub struct SysEnter;

impl Program for SysEnter {
    fn run(&self, stop: Arc<AtomicBool>, interval: Duration) {
        while !stop.load(Ordering::SeqCst) {
            println!("sys_enter");
            thread::sleep(interval);
        }
    }
}
