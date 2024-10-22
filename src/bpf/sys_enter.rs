use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use prometheus_client::{
    collector::Collector,
    encoding::{DescriptorEncoder, EncodeMetric},
    metrics::{family::Family, gauge::Gauge},
};

use crate::metrics::labels::ProcessLabels;

#[derive(Debug)]
pub struct SysEnter {
    counts: Family<ProcessLabels, Gauge>,
}

impl SysEnter {
    pub fn new() -> Self {
        Self {
            counts: Family::<ProcessLabels, Gauge>::default(),
        }
    }
}

impl super::Program for SysEnter {
    fn run(&self, stop: Arc<AtomicBool>, interval: Duration) {
        while !stop.load(Ordering::SeqCst) {
            println!("sys_enter");

            // TODO: get actual sys_enter count.
            self.counts
                .get_or_create(&ProcessLabels {
                    pid: 1,
                    pname: "pname_a".to_string(),
                })
                .set(10);

            thread::sleep(interval);
        }
    }
}

impl Collector for SysEnter {
    fn encode(&self, mut encoder: DescriptorEncoder) -> Result<(), std::fmt::Error> {
        let metric_encoder = encoder.encode_descriptor(
            "sys_enter_total",
            "Number of syscall entries",
            None,
            self.counts.metric_type(),
        )?;
        self.counts.encode(metric_encoder)?;
        Ok(())
    }
}
