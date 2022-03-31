use anyhow::Error;
use log::*;
use simplelog::*;

/// Increase the RLIMIT_MEMLOCK in order to load bpf programs.
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

/// Configure structured logging for use when running as a system service.
pub fn configure_loging(level: &str) {
    // Parse given log level and default to info.
    let max_lvl = match level.to_lowercase().as_str() {
        "debug" => LevelFilter::Debug,
        "error" => LevelFilter::Error,
        "info" => LevelFilter::Info,
        _ => LevelFilter::Info,
    };

    // TODO: configure timestamp.
    let config = ConfigBuilder::new().build();

    TermLogger::init(max_lvl, config, TerminalMode::Mixed, ColorChoice::Never).unwrap();
}
