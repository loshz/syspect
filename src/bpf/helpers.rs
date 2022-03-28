use anyhow::Error;

pub fn remove_memlock_rlimit() -> Result<(), Error> {
    let rlimit = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };

    if unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlimit) } != 0 {
        return Err(Error::msg("failed to increase rlimit".to_owned()));
    }

    Ok(())
}
