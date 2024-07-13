use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A specialized `Result` type for the `zerodb` CLI.
pub type Result<T> = std::result::Result<T, ZerodbCliError>;

/// Errors that can occur while running the `zerodb` CLI.
#[derive(Debug, Error)]
pub enum ZerodbCliError {
    /// An error from the `zerodb` engine.
    #[error(transparent)]
    Zerodb(#[from] zerodb::ZerodbError),

    /// An I/O error.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// A TOML error.
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),

    /// A UUID error.
    #[error(transparent)]
    NodeId(#[from] zerodb::raft::NodeIdError),

    /// A SocketAddr error.
    #[error(transparent)]
    AddrParse(#[from] std::net::AddrParseError),
}
