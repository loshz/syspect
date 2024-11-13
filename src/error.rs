/// Wrapped operation errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error: invalid config: {0}")]
    Config(String),

    #[error("error: {0}")]
    IO(#[from] std::io::Error),

    #[error("error: root privileges required")]
    PermissionDenied,

    #[error("error: encountered bpf program issue: {0}")]
    Program(#[from] ProgramError),
}

/// BPF program specific errors.
#[derive(Debug, thiserror::Error)]
pub enum ProgramError {
    #[error("failed to open program `{0}`")]
    Open(&'static str),

    #[error("failed to load program `{0}`")]
    Load(&'static str),

    #[error("failed to attach to program `{0}`")]
    Attach(&'static str),

    #[error("unsupported program `{0}`")]
    Unsupported(String),
}
