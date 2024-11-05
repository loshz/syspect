use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::{
    sync::{Arc, RwLock},
    thread,
};

use metrics_server::MetricsServer;
use prometheus_client::{
    encoding::{text::encode, DescriptorEncoder},
    registry::{Metric, Registry},
};

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

pub struct Collector {
    // The Prometheus registry to store metrics.
    registry: Arc<RwLock<Registry>>,
    // The join handle of the running collector thread.
    thread: Option<thread::JoinHandle<()>>,
}

impl Collector {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(<Registry>::default())),
            thread: None,
        }
    }

    pub fn register(&mut self, collector: Box<dyn prometheus_client::collector::Collector>) {
        self.registry.write().unwrap().register_collector(collector);
    }

    pub fn start(&mut self, stop: Arc<AtomicBool>, addr: &str) {
        let server = MetricsServer::http(addr);
        let registry = Arc::clone(&self.registry);

        self.thread = Some(thread::spawn({
            move || {
                while !stop.load(Ordering::SeqCst) {
                    // Encode the current registry and update the metrics server every 3 seconds.
                    let mut encoded = String::new();
                    let r = registry.read().unwrap();
                    encode(&mut encoded, &r).unwrap();
                    server.update(encoded.into_bytes());
                    thread::sleep(Duration::from_secs(3));
                }
            }
        }));
    }

    pub fn stop(mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().ok();
        }
        // TODO: call server.stop()
    }
}
