use std::path::Path;

use crate::Error;

pub fn run(_verbose: bool) -> Result<(), Error> {
    crate::is_root()?;

    println!("Currently available Kernel trace events:");

    println!(
        "- sys_enter: {}",
        Path::new("/sys/kernel/debug/tracing/events/raw_syscalls/sys_enter").is_dir()
    );

    Ok(())
}
