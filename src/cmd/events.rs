use std::path::Path;

use crate::{cmd::is_root, Error};

pub fn run(_verbose: bool) -> Result<(), Error> {
    is_root()?;

    println!("Currently available Kernel trace events:");

    println!(
        "- sys_enter: {}",
        Path::new("/sys/kernel/debug/tracing/events/raw_syscalls/sys_enter").is_dir()
    );

    Ok(())
}
