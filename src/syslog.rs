use log::LevelFilter;
use syslog::{BasicLogger, Facility, Formatter3164};

/// Creates a new system logger and sets it as the default logger.
pub fn init_syslog(module_name: &str) {
    // Hostname and PID will be populated automatically.
    let formatter = Formatter3164 {
        facility: Facility::LOG_AUTHPRIV,
        process: module_name.into(),
        ..Formatter3164::default()
    };

    // Create a Logger using a unix socket to target local syslog and
    // set it as the global logger with a max level.
    let _ = match syslog::unix(formatter) {
        Ok(logger) => log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
            .map(|()| log::set_max_level(LevelFilter::Info)),
        Err(e) => {
            eprintln!("error connecting to syslog: {:?}", e);
            return;
        }
    };
}
