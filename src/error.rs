/// Wrapped operation errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error: invalid config: {0}")]
    Config(String),

    #[error("TODO")]
    IO(#[from] std::io::Error),

    #[error("error: root privileges required")]
    PermissionDenied,

    #[error("error: encountered issue running bpf program: {0}")]
    Program(String),

    #[error("error: unsupported event type `{0}`")]
    UnsopprtedProgram(String),
}
