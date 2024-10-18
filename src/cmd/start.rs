use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use metrics_server::MetricsServer;
use prometheus_client::{encoding::text::encode, registry::Registry};

use crate::{bpf, config::Config, Error};

pub fn run(config_path: &str) -> Result<(), Error> {
    // Load config from file.
    let c = Config::from_file(config_path)?;
    println!(
        "Starting service: {} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );
    println!("Using config: {}", config_path);

    // Configure and expose a metrics registry.
    let registry = <Registry>::default();
    let server = MetricsServer::http(&c.metrics_addr);
    println!("Metrics exposed at: http://{}/metrics", &c.metrics_addr);

    // Register CTRL-C handler.
    let stop = Arc::new(AtomicBool::new(false));
    let s = stop.clone();
    ctrlc::set_handler(move || {
        println!("Stopping...");
        s.store(true, Ordering::SeqCst);
    })
    .unwrap();

    let programs = bpf::parse_event_programs(c.tracing.events);
    let join_handles: Vec<_> = programs
        .into_iter()
        .map(|program| {
            let s = stop.clone();
            thread::spawn(move || {
                program.run(s, Duration::from_secs(c.tracing.interval));
            })
        })
        .collect();

    // Encode the current registry and update the metrics server.
    let mut encoded = String::new();
    encode(&mut encoded, &registry).unwrap();
    server.update(encoded.into_bytes());

    // TODO: is there a better way of blocking here?
    while !stop.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(1));
    }

    // Gracefully shut down.
    let _ = join_handles.into_iter().map(|thread| thread.join().ok());
    let _ = server.stop();

    Ok(())
}
