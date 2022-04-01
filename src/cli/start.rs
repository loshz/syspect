use std::time::Duration;
use std::u64;

use anyhow::{Context, Error};
use clap::ArgMatches;
use log::info;
use metrics_server::MetricsServer;
use prometheus_client::encoding::text::encode;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use tokio::{signal, time};

use crate::bpf;
use crate::config;
use crate::helpers;

/// The command name.
pub const COMMAND_NAME: &str = "start";

#[tokio::main]
pub async fn run(args: &ArgMatches) -> Result<(), Error> {
    let path = match args.value_of("config") {
        Some(path) => path,
        _ => return Err(Error::msg("config not specified".to_owned())),
    };

    // Load config from file.
    let c = config::from_file(path)?;

    // Configure system logging.
    helpers::configure_loging(&c.log_level);
    info!(
        "Starting service: {} {}",
        crate::PKG_NAME,
        crate::PKG_VERSION
    );
    info!("Using config: {}", path);

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
    let server = MetricsServer::new();
    let addr = c.metrics_address();
    server.serve(addr.as_str());
    info!("Metrics exposed on: http://{}", addr.as_str());

    // Load the eBPF program and attach to the tracepoint.
    let sys_enter = bpf::sys_enter::SysEnterSkelBuilder::default();
    let prog = sys_enter.open().context("failed to open ebpf prog")?;
    let mut tracepoint = prog.load().context("failed to load ebpf prog")?;
    tracepoint
        .attach()
        .context("failed to attach to ebpf tracepoint")?;

    loop {
        // Listen for CTRL+C signal events or sleep.
        tokio::select! {
            _ = time::sleep(Duration::from_secs(1)) => {}
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
                gauge.inc_by(count);
                let mut encoded = Vec::new();
                encode(&mut encoded, &registry).unwrap();
                server.update(encoded);
            }
        }
    }

    info!("Stopping service");

    Ok(())
}
