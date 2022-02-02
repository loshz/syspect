use std::process;

use log::error;
use nix::unistd::Uid;

mod syslog;

use crate::syslog::init_syslog;

/// The service name.
const SERVICE_NAME: &str = "lemurs";

fn main() {
    // Initialize system logging.
    init_syslog(SERVICE_NAME);

    // Check if ran as root.
    if !Uid::effective().is_root() {
        eprintln!("This service requires root permissions");
        process::exit(1);
    }

    // Run the service.
    process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            error!("error: {:?}", err);
            1
        }
    });
}

fn run() -> Result<(), ()> {
    Ok(())
}
