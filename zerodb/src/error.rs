//! Error types of the zerodb crate.

use crate::client::ClientRequest;
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

    /// Tokio Client Request channel error.
    #[error(transparent)]
    TokioChannelError(#[from] tokio::sync::mpsc::error::SendError<ClientRequest>),

    /// Tokio Raft Request channel error.
    #[error(transparent)]
    TokioRaftChannelError(#[from] tokio::sync::mpsc::error::SendError<crate::raft::RaftRequest>),

    /// CBOR decode error.
    #[error(transparent)]
    DecoderError(#[from] cbor4ii::serde::DecodeError<Infallible>),

    /// CBOR encode error.
    #[error(transparent)]
    EncoderError(#[from] cbor4ii::serde::EncodeError<TryReserveError>),
}
