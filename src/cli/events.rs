use std::path::Path;

use anyhow::Error;

pub fn run(_verbose: bool) -> Result<(), Error> {
    println!("Currently available Kernel trace events:");

    println!(
        "- sys_enter: {}",
        Path::new("/sys/kernel/debug/tracing/events/raw_syscalls/sys_enter").is_dir()
    );

    Ok(())
}
