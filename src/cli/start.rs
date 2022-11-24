use std::time::Duration;
use std::{u32, u64};

use anyhow::{Context, Error};
use log::info;
use metrics_server::MetricsServer;
use prometheus_client::encoding::text::{encode, Encode};
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use tokio::{signal, time};

use crate::{bpf, config, helpers};

#[derive(Clone, Hash, PartialEq, Eq, Encode)]
struct Labels {
    // Process ID
    pid: u32,
    // Process name
    pname: String,
}

#[tokio::main]
pub async fn run(config_path: &str, syslog: bool) -> Result<(), Error> {
    // Load config from file.
    let c = config::from_file(config_path)?;

    // Configure system logging.
    helpers::configure_loging(&c.log_level, syslog);
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
    let sys_enter_total = Family::<Labels, Gauge>::default();
    registry.register(
        "sys_enter_total",
        "Number of syscall entries",
        sys_enter_total.clone(),
    );

    // Expose the Prometheus metrics.
    let addr = c.metrics_address();
    let server = MetricsServer::http(addr.as_str());
    info!("Metrics exposed on: http://{}/metrics", addr.as_str());

    // Load the eBPF program and attach to the tracepoint.
    let sys_enter = bpf::sys_enter::SysEnterSkelBuilder::default();
    let prog = sys_enter.open().context("failed to open ebpf prog")?;
    let mut tracepoint = prog.load().context("failed to load ebpf prog")?;
    tracepoint
        .attach()
        .context("failed to attach to ebpf tracepoint")?;
    info!("Tracing Kernel events every {}s", c.interval);

    loop {
        // Listen for CTRL+C signal events or sleep.
        tokio::select! {
            _ = time::sleep(Duration::from_secs(c.interval)) => {}
            _ = signal::ctrl_c() => break
        }

        for key in tracepoint.maps().syscall_count().keys() {
            let syscall_count = match tracepoint
                .maps()
                .syscall_count()
                .lookup(&key, libbpf_rs::MapFlags::ANY)
            {
                Ok(count) => count,
                Err(e) => return Err(Error::new(e)),
            };

            // Check that count contains Some value.
            // TODO: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=eaac5faf4aae5f88fd32f0dab784336a
            if let Some(total) = syscall_count {
                let pname = String::from_utf8_lossy(&key[0..16]);
                let mut pid = [0u8; 4];
                pid.clone_from_slice(&key[16..20]);
                let count = u64::from_le_bytes(total.try_into().unwrap());
                println!("@[{}, {:?}]: {}", pname, u32::from_le_bytes(pid), count);

                sys_enter_total
                    .get_or_create(&Labels {
                        pid: u32::from_le_bytes(pid),
                        pname: pname.to_string(),
                    })
                    .set(count);
            }
        }

        // Encode the current registry and update the metrics server.
        let mut encoded = Vec::new();
        encode(&mut encoded, &registry).unwrap();
        server.update(encoded);
    }

    info!("Terminating...");
    let _ = server.stop();

    Ok(())
}
