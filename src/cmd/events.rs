use std::{fs, path::PathBuf};

use crate::{bpf::SUPPORTED_EVENTS, cmd::is_root, Error};

pub fn run() -> Result<(), Error> {
    is_root()?;

    // Read available tracing events.
    let available_events = PathBuf::from("/sys/kernel/debug/tracing/available_events");
    let events = fs::read_to_string(available_events.as_path()).map_err(Error::IO)?;

    println!("Currently supported Kernel tracing events:");
    SUPPORTED_EVENTS.iter().for_each(|event| {
        if events.contains(event) {
            println!("- {}", event);
        }
    });

    Ok(())
}
