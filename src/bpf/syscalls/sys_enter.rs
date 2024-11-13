use std::mem::MaybeUninit;

use libbpf_rs::skel::{OpenSkel, Skel, SkelBuilder};
use prometheus_client::{
    collector::Collector,
    metrics::{family::Family, gauge::Gauge},
};

use crate::{
    bpf::Programmable,
    metrics::{labels::ProcessLabels, Collectable},
    ProgramError,
};

// Include the generated bpf skeleton.
// include!(concat!(env!("OUT_DIR"), "/sys_enter.bpf.rs"));

const PROGRAM_NAME: &str = "sys_enter";

#[derive(Debug)]
pub struct SysEnter {
    totals: Family<ProcessLabels, Gauge>,
}

impl Programmable for SysEnter {
    fn new() -> Self {
        Self {
            totals: Family::<ProcessLabels, Gauge>::default(),
        }
    }

    fn run(&self) -> Result<(), ProgramError> {
        println!("sys_enter");

        let sys_enter = SysEnterSkelBuilder::default();
        let mut open_object = MaybeUninit::uninit();
        let prog = sys_enter
            .open(&mut open_object)
            .map_err(|_| ProgramError::Open(PROGRAM_NAME))?;
        let mut tracepoint = prog.load().map_err(|_| ProgramError::Load(PROGRAM_NAME))?;
        tracepoint
            .attach()
            .map_err(|_| ProgramError::Attach(PROGRAM_NAME))?;

        // TODO: get actual sys_enter count.
        self.totals
            .get_or_create(&ProcessLabels {
                pid: 1,
                pname: "pname_a".to_string(),
            })
            .set(10);

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
