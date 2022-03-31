use std::time::Duration;
use std::u64;

use anyhow::{Context, Error};
use clap::ArgMatches;
use metrics_server::MetricsServer;
use tokio::{signal, time};

use crate::bpf;
use crate::config;
use crate::helpers;

/// The command name.
pub const COMMAND_NAME: &str = "start";

#[tokio::main]
pub async fn run(args: &ArgMatches) -> Result<(), Error> {
    println!(
        "Starting service, version: {} {}",
        crate::PKG_NAME,
        crate::PKG_VERSION
    );

    let path = match args.value_of("config") {
        Some(path) => path,
        _ => return Err(Error::msg("config not specified".to_owned())),
    };
    println!("Using config: {}", path);

    let c = config::from_file(path)?;

    // Remove memlock limit in order to load eBPF programs.
    helpers::remove_memlock_rlimit()?;
    println!("removed memlock rlimit");

    // Expose the Prometheus metrics.
    let server = MetricsServer::new();
    server.serve(c.metrics_address().as_str());

    let sys_enter = bpf::sys_enter::SysEnterSkelBuilder::default();
    let prog = sys_enter.open().context("failed to open bpf prog")?;
    let mut tracepoint = prog.load().context("failed to load bpf prog")?;
    tracepoint
        .attach()
        .context("failed to attach to bpf tracepoint")?;

    loop {
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
                // total is a &[u8; 8] containing the little-endian representation of a 64 bit number.
                println!("{}", u64::from_le_bytes(total.try_into().unwrap()));
            }
        }
    }

    // TODO: do something on shutdown.

    Ok(())
}
