//! Error types of the zerodb crate.

use std::{collections::TryReserveError, convert::Infallible};

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

    /// A TOML error.
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),

    /// Http error from `hyper` crate.
    #[error(transparent)]
    HyperHttpError(#[from] hyper::http::Error),

    /// Tokio join error.
    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),

    /// CBOR decode error.
    #[error(transparent)]
    DecoderError(#[from] cbor4ii::serde::DecodeError<Infallible>),

    /// CBOR encode error.
    #[error(transparent)]
    EncoderError(#[from] cbor4ii::serde::EncodeError<TryReserveError>),

    /// Tokio channel send error.
    #[error("tokio channel send error: {0}")]
    TokioSendError(String),

    /// TODO
    #[error("TODO")]
    Todo,
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for ZerodbError {
    fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::TokioSendError(err.to_string())
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
