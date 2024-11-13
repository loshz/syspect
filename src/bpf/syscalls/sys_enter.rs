use std::mem::MaybeUninit;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use libbpf_rs::skel::{OpenSkel, Skel, SkelBuilder};
use prometheus_client::{
    collector::Collector,
    metrics::{family::Family, gauge::Gauge},
};

use crate::{
    bpf::Program,
    metrics::{labels::ProcessLabels, Collectable},
    ProgramError,
};

// Include the generated bpf skeleton.
include!(concat!(env!("OUT_DIR"), "/sys_enter.bpf.rs"));

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
        let prog = sys_enter
            .open(&mut open_object)
            .map_err(|_| ProgramError::Open(SYS_ENTER))?;
        let mut tracepoint = prog.load().map_err(|_| ProgramError::Load(SYS_ENTER))?;
        tracepoint
            .attach()
            .map_err(|_| ProgramError::Attach(SYS_ENTER))?;

        // TODO: get actual sys_enter count.
        self.totals
            .get_or_create(&ProcessLabels {
                pid: 1,
                pname: "pname_a".to_string(),
            })
            .set(10);

        while !stop.load(Ordering::SeqCst) {
            println!("sys_enter");
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
