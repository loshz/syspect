use std::time::Duration;

use metrics_server::MetricsServer;
use prometheus_client::{encoding::text::encode, registry::Registry};
use tokio::{signal, time};

use crate::{config::Config, Error};

#[tokio::main]
pub async fn run(config_path: &str) -> Result<(), Error> {
    // Load config from file.
    let c = Config::from_file(config_path)?;
    println!("Using config: {}", config_path);
    println!(
        "Starting service: {} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    // Configure metrics.
    let registry = <Registry>::default();

    // Expose the Prometheus metrics.
    let addr = c.metrics_addr.as_str();
    let server = MetricsServer::http(addr);
    println!("Metrics exposed at: http://{}/metrics", addr);

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

    println!("Terminating...");
    if let Err(e) = server.stop() {
        eprintln!("{e}");
    }

    Ok(())
}
