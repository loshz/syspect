use std::time::Duration;

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

        let b: &[u8; 4] = &[0, 0, 0, 0];
        let count = tracepoint
            .maps()
            .syscall_count()
            .lookup(b, libbpf_rs::MapFlags::ANY);

        let total = match count {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}", e);
                return Err(Error::msg("total".to_owned()));
            }
        };

        if let Some(t) = total {
            println!("{:?}", t.get(0));
        }
    }

    // TODO: do something on shutdown.

    Ok(())
}
