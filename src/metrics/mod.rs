use prometheus_client::{encoding::DescriptorEncoder, registry::Metric};

pub mod collector;
pub mod labels;

#[derive(Debug)]
pub struct Collectable<M: Metric> {
    name: &'static str,
    help: &'static str,
    metric: M,
}

impl<M: Metric> Collectable<M> {
    pub fn new(name: &'static str, help: &'static str, metric: M) -> Self {
        Self { name, help, metric }
    }
}

impl<M: Metric> prometheus_client::collector::Collector for Collectable<M> {
    fn encode(&self, mut encoder: DescriptorEncoder) -> Result<(), std::fmt::Error> {
        let metric_encoder =
            encoder.encode_descriptor(self.name, self.help, None, self.metric.metric_type())?;
        self.metric.encode(metric_encoder)?;
        Ok(())
    }
}
