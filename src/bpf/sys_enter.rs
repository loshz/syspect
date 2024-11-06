use prometheus_client::{
    collector::Collector,
    metrics::{family::Family, gauge::Gauge},
};

use crate::{
    metrics::{labels::ProcessLabels, Collectable},
    Error,
};

use super::Programmable;

// Include the generated bpf skeleton.
include!(concat!(env!("OUT_DIR"), "/sys_enter.bpf.rs"));

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

    fn run(&self) -> Result<(), Error> {
        println!("sys_enter");

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
