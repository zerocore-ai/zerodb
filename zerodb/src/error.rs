//! Error types of the zerodb crate.

use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A specialized `Result` type for zeroengine crate.
pub type Result<T> = std::result::Result<T, ZerodbError>;

/// The main error type of the zeroengine crate.
#[derive(Debug, Error)]
pub enum ZerodbError {
    /// Unsupported equal peer and client ports.
    #[error("unsupported equal peer and client ports: {0} == {0}")]
    EqualPeerClientPorts(u16),

    /// Failed to parse the HTTP address.
    #[error("failed to parse HTTP address: {0}")]
    AddrParse(#[from] std::net::AddrParseError),

    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Cannot upgrade to websocket connection for peer connection.
    #[error("cannot upgrade to websocket connection for peer connection")]
    CannotUpgradePeerConnection,

    /// Error from hyper_tungstenite.
    #[error(transparent)]
    WebsocketError(#[from] hyper_tungstenite::tungstenite::Error),

    /// Protocol error from tungstenite.
    #[error(transparent)]
    ProtocolError(#[from] hyper_tungstenite::tungstenite::error::ProtocolError),

    /// A TOML error.
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),

    /// Http error from `hyper` crate.
    #[error(transparent)]
    HyperHttpError(#[from] hyper::http::Error),

    /// Tokio join error.
    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),
}
