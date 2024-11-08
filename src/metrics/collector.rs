use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::{
    sync::{Arc, RwLock},
    thread,
};

use metrics_server::MetricsServer;
use prometheus_client::{encoding::text::encode, registry::Registry};

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

    pub fn register(&mut self, collectors: Vec<Box<dyn prometheus_client::collector::Collector>>) {
        let mut registry = self.registry.write().unwrap();
        collectors
            .into_iter()
            .for_each(|collector| registry.register_collector(collector));
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
