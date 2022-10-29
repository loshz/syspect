use std::time::Duration;
use std::u64;

use anyhow::{Context, Error};
use log::info;
use metrics_server::MetricsServer;
use prometheus_client::encoding::text::encode;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use tokio::{signal, time};

use crate::{bpf, config, helpers};

#[tokio::main]
pub async fn run(config_path: &str) -> Result<(), Error> {
    // Load config from file.
    let c = config::from_file(config_path)?;

    // Configure system logging.
    helpers::configure_loging(&c.log_level);
    info!("Using config: {}", config_path);
    info!(
        "Starting service: {} {}",
        crate::PKG_NAME,
        crate::PKG_VERSION
    );

    // Remove memlock limit in order to load eBPF programs.
    helpers::remove_memlock_rlimit()?;
    info!("Removed memlock rlimit");

    // Configure metrics
    let mut registry = Registry::default();
    let gauge: Gauge = Gauge::default();
    registry.register(
        "sys_enter_total",
        "Number of syscall entries",
        gauge.clone(),
    );

    // Expose the Prometheus metrics.
    let addr = c.metrics_address();
    let server = MetricsServer::http(addr.as_str());
    info!("Metrics exposed on: http://{}", addr.as_str());

    // Load the eBPF program and attach to the tracepoint.
    let sys_enter = bpf::sys_enter::SysEnterSkelBuilder::default();
    let prog = sys_enter.open().context("failed to open ebpf prog")?;
    let mut tracepoint = prog.load().context("failed to load ebpf prog")?;
    tracepoint
        .attach()
        .context("failed to attach to ebpf tracepoint")?;
    info!("Running eBPF programs every {}s", c.interval);

    // Keep track of the current count.
    let mut current = 0;

    loop {
        // Listen for CTRL+C signal events or sleep.
        tokio::select! {
            _ = time::sleep(Duration::from_secs(c.interval)) => {}
            _ = signal::ctrl_c() => break
        }

        // Lookup the value saved to the bpf map.
        // This will return a Result<Option<Vec<u8>>>
        let b: &[u8; 4] = &[0, 0, 0, 0];
        let syscall_count = match tracepoint
            .maps()
            .syscall_count()
            .lookup(b, libbpf_rs::MapFlags::ANY)
        {
            Ok(count) => count,
            Err(e) => return Err(Error::new(e)),
        };

        // Check that count contains Some value.
        // Continue if no count.
        if let Some(total) = syscall_count {
            if !total.is_empty() {
                // total is a &[u8; 8] containing the little-endian representation of a 64 bit number,
                // so we need to convert to u64 in order to get the count.
                let count = u64::from_le_bytes(total.try_into().unwrap());

                // Update the gauge and current count value.
                gauge.set(count - current);
                current = count;

                // Encode the current registry and update the metrics server.
                let mut encoded = Vec::new();
                encode(&mut encoded, &registry).unwrap();
                server.update(encoded);
            }
        }
    }

    info!("Terminating...");
    let _ = server.stop();

    Ok(())
}
