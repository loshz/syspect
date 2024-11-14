use std::mem::MaybeUninit;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use libbpf_rs::skel::{MapFlags, OpenSkel, Skel, SkelBuilder};
use prometheus_client::{
    collector::Collector,
    metrics::{family::Family, gauge::Gauge},
};

use crate::{
    bpf::Program,
    include_bpf,
    metrics::{labels::ProcessLabels, Collectable},
    ProgramError,
};

// Include the generated bpf skeleton.
include_bpf!("sys_enter");

pub(crate) const SYS_ENTER: &str = "sys_enter";

#[derive(Debug)]
pub(crate) struct SysEnter {
    totals: Family<ProcessLabels, Gauge>,
}

impl Program for SysEnter {
    fn new() -> Self {
        Self {
            totals: Family::<ProcessLabels, Gauge>::default(),
        }
    }

    fn run(&self, interval: Duration, stop: Arc<AtomicBool>) -> Result<(), ProgramError> {
        let sys_enter = SysEnterSkelBuilder::default();
        let mut open_object = MaybeUninit::uninit();
        let tracepoint = sys_enter
            .open(&mut open_object)
            .map_err(|_| ProgramError::Open(SYS_ENTER))?
            .load()
            .map_err(|_| ProgramError::Load(SYS_ENTER))?
            .attach()
            .map_err(|_| ProgramError::Attach(SYS_ENTER))?;

        while !stop.load(Ordering::SeqCst) {
            let map = tracepoint.maps().syscall_count();

            for key in map.keys() {
                // TODO: log error.
                let Ok(syscall_count) = map.lookup(&key, MapFlags::ANY) else {
                    continue;
                };
            }

            thread::sleep(interval);
        }

        Ok(())
    }

    fn metrics(&self) -> Vec<Box<dyn Collector>> {
        vec![Box::new(Collectable::new(
            "sys_enter_total",
            "Number of syscall entries",
            self.totals.clone(),
        ))]
    }
}
