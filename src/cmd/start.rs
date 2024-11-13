use std::str::FromStr;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use crate::{bpf::Program, config::Config, metrics::collector::Collector, Error};

pub fn run(config_path: &str) -> Result<(), Error> {
    println!(
        "Starting service: {} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    // Load config from file.
    let c = Config::from_file(config_path)?;
    println!("Using config: {}", config_path);

    // Register CTRL-C handler.
    let stop = Arc::new(AtomicBool::new(false));
    let s = stop.clone();
    ctrlc::set_handler(move || {
        s.store(true, Ordering::SeqCst);
    })
    .unwrap();

    // Create a new metrics collector.
    let mut collector = Collector::new();

    // Parse trace events and start individual bpf programs.
    let join_handles: Vec<_> = c
        .tracing
        .events
        .into_iter()
        .filter_map(|event| match Box::<dyn Program>::from_str(&event) {
            Ok(program) => {
                // Register the program metrics with the collector.
                collector.register(program.metrics());

                // Start the program in the background.
                let s = stop.clone();
                let handle = thread::spawn(move || program.run(c.tracing.interval, s));
                Some(handle)
            }
            Err(e) => {
                eprintln!("{e}");
                None
            }
        })
        .collect();

    // If we don't have any running programs, we can assume there was an issue.
    if join_handles.is_empty() {
        return Err(Error::Config("unable to parse any bpf programs".into()));
    }

    // After loading the bpf programs, start the collector.
    collector.start(stop.clone(), &c.metrics_addr);
    println!("Metrics exposed at: http://{}/metrics", &c.metrics_addr);

    // TODO: is there a better way of blocking here?
    while !stop.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(1));
    }

    // Gracefully shut down.
    let _ = join_handles.into_iter().map(|thread| thread.join().ok());
    collector.stop();

    Ok(())
}
