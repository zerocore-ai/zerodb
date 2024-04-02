//! Error types of the zerodb crate.

use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A specialized `Result` type for zerodb crate.
pub type Result<T> = std::result::Result<T, ZerodbError>;

/// The main error type of the zerodb crate.
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

    /// A TOML error.
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),

    /// Http error from `hyper` crate.
    #[error(transparent)]
    HyperHttpError(#[from] hyper::http::Error),

    /// Tokio join error.
    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),

    /// Tokio channel send error.
    #[error("tokio channel send error: {0}")]
    TokioSendError(String),

    /// Error from zeroraft crate
    #[error(transparent)]
    ZeroraftError(#[from] zeroraft::ZeroraftError),

    /// Error decoding with cbor4ii
    #[error("cbor4ii decode error: {0}")]
    Cbor4iiDecodeError(String),

    /// Error encoding with cbor4ii
    #[error("cbor4ii encode error: {0}")]
    Cbor4iiEncodeError(String),

    /// Peer not found.
    #[error("peer not found")]
    PeerNotFound,

    /// Channel closed.
    #[error("channel closed")]
    ChannelClosed,
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for ZerodbError {
    fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::TokioSendError(err.to_string())
    }
}

impl<T> From<cbor4ii::serde::DecodeError<T>> for ZerodbError
where
    T: std::fmt::Debug,
{
    fn from(err: cbor4ii::serde::DecodeError<T>) -> Self {
        Self::Cbor4iiDecodeError(format!("{:?}", err))
    }
}

impl<T> From<cbor4ii::serde::EncodeError<T>> for ZerodbError
where
    T: std::fmt::Debug,
{
    fn from(err: cbor4ii::serde::EncodeError<T>) -> Self {
        Self::Cbor4iiEncodeError(format!("{:?}", err))
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates a new `Ok` result.
#[allow(non_snake_case)]
pub fn Ok<T>(value: T) -> Result<T> {
    Result::Ok(value)
}
