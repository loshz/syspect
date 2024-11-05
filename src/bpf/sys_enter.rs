use prometheus_client::{
    collector::Collector,
    encoding::{DescriptorEncoder, EncodeMetric},
    metrics::{family::Family, gauge::Gauge},
};

use crate::{metrics::labels::ProcessLabels, Error};

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
