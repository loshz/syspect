use std::mem::MaybeUninit;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

use libbpf_rs::{
    skel::{OpenSkel, Skel, SkelBuilder},
    MapCore, MapFlags,
};
use prometheus_client::{
    collector::Collector,
    metrics::{family::Family, gauge::Gauge},
};

use crate::{
    bpf::{ffi::Process, Program, ProgramOptions},
    metrics::{labels::ProcessLabels, Collectable},
    ProgramError,
};

// Include the generated bpf skeleton.
mod bpf {
    include!(concat!(env!("OUT_DIR"), "/sys_enter.bpf.rs"));
}

pub(crate) const SYS_ENTER: &str = "sys_enter";

#[derive(Debug)]
pub(crate) struct SysEnter {
    counts: Family<ProcessLabels, Gauge>,
}

impl Program for SysEnter {
    fn new() -> Self {
        Self {
            counts: Family::<ProcessLabels, Gauge>::default(),
        }
    }

    fn run(&self, opts: ProgramOptions, stop: Arc<AtomicBool>) -> Result<(), ProgramError> {
        let mut sys_enter = bpf::SysEnterSkelBuilder::default();
        if opts.debug {
            sys_enter.obj_builder.debug(true);
        }

        // Attempt to open and load the program into the kernel.
        let mut open_object = MaybeUninit::uninit();
        let mut tracepoint = sys_enter
            .open(&mut open_object)
            .map_err(|_| ProgramError::Open(SYS_ENTER))?
            .load()
            .map_err(|_| ProgramError::Load(SYS_ENTER))?;

        // Attempt the program to the kernel hook.
        tracepoint
            .attach()
            .map_err(|_| ProgramError::Attach(SYS_ENTER))?;

        while !stop.load(Ordering::SeqCst) {
            let map = &tracepoint.maps.syscall_count;
            map.keys().for_each(|key| {
                // Read the entry from the map.
                let Ok(value) = map.lookup(&key, MapFlags::ANY) else {
                    // TODO: log error.
                    return;
                };

                // If we successfully received bytes, attempt to parse them into a process.
                if let Some(syscall_count) = value {
                    let process = Process::from(key);
                    let labels = ProcessLabels::from(process);
                    let count = i64::from_le_bytes(syscall_count.try_into().unwrap_or_default());

                    // Update the count for the given process.
                    self.counts.get_or_create(&labels).set(count);
                }
            });

            thread::sleep(opts.interval);
        }

        Ok(())
    }

    fn metrics(&self) -> Vec<Box<dyn Collector>> {
        vec![Box::new(Collectable::new(
            "sys_enter_total",
            "Number of syscall entries",
            self.counts.clone(),
        ))]
    }
}
