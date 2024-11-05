use prometheus_client::{
    collector::Collector,
    metrics::{family::Family, gauge::Gauge},
};

use crate::{
    metrics::{labels::ProcessLabels, Collectable},
    Error,
};

use super::Programmable;

#[derive(Debug)]
pub struct SysEnter {
    counts: Family<ProcessLabels, Gauge>,
}

impl Programmable for SysEnter {
    fn new() -> Self {
        Self {
            counts: Family::<ProcessLabels, Gauge>::default(),
        }
    }

    fn run(&self) -> Result<(), Error> {
        println!("sys_enter");

        // TODO: get actual sys_enter count.
        self.counts
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
            self.counts.clone(),
        ))]
    }
}
