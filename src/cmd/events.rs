use std::path::{Path, PathBuf};

use crate::{bpf::SUPPORTED_EVENTS, cmd::is_root, Error};

pub fn run() -> Result<(), Error> {
    is_root()?;

    // debugfs path.
    let debugfs = PathBuf::from("/sys/kernel/debug/tracing/events");

    println!("Currently supported Kernel tracing events:");
    SUPPORTED_EVENTS.iter().for_each(|event| {
        let event_path = PathBuf::from(&debugfs).join(event.replace(":", "/"));
        if Path::new(event_path.as_path()).is_dir() {
            println!("- {}", event);
        }
    });

    Ok(())
}
