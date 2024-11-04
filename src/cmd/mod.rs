use nix::unistd::Uid;

use crate::Error;

mod cli;
mod events;
mod install;
mod start;
mod uninstall;

pub use cli::Cli;

/// Retuns `Ok` if current use is root.
pub(crate) fn is_root() -> Result<(), Error> {
    Uid::current()
        .is_root()
        .then_some(())
        .ok_or(Error::PermissionDenied)
}
