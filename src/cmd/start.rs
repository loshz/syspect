use std::time::Duration;

use log::info;
use metrics_server::MetricsServer;
use prometheus_client::{
    encoding::{text::encode, EncodeLabelSet},
    registry::Registry,
};
use tokio::{signal, time};

use crate::{config::Config, Error};

#[derive(Clone, Hash, PartialEq, Eq, EncodeLabelSet)]
struct Labels {
    // Process ID
    pid: u32,
    // Process name
    pname: String,
}

#[tokio::main]
pub async fn run(config_path: &str) -> Result<(), Error> {
    // Load config from file.
    let c = Config::from_file(config_path)?;

    // Configure system logging.
    crate::configure_loging(&c.log_level, c.syslog);
    info!("Using config: {}", config_path);
    info!(
        "Starting service: {} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    // Configure metrics.
    let registry = <Registry>::default();

    // Expose the Prometheus metrics.
    let addr = c.metrics_addr.as_str();
    let server = MetricsServer::http(addr);
    info!("Metrics exposed at: http://{}/metrics", addr);

    let poll_interval = Duration::from_secs(c.tracing.interval);
    loop {
        // Listen for CTRL+C signal events or sleep.
        tokio::select! {
            _ = time::sleep(poll_interval) => {}
            _ = signal::ctrl_c() => break
        }

        // Encode the current registry and update the metrics server.
        let mut encoded = String::new();
        encode(&mut encoded, &registry).unwrap();
        server.update(encoded.into_bytes());
    }

    info!("Terminating...");
    let _ = server.stop();

    Ok(())
}
