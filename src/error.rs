/// Wrapped operation errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid config: {0}")]
    Config(String),

    #[error("TODO")]
    IO(#[from] std::io::Error),

    #[error("root privileges required")]
    PermissionDenied,
}
