//! Error types of the zeroraft crate.

use std::fmt::Display;

use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A specialized `Result` type for zeroengine crate.
pub type Result<T> = std::result::Result<T, ZeroraftError>;

/// The main error type of the zeroengine crate.
#[derive(Debug, Error)]
pub enum ZeroraftError {
    /// Custom error.
    #[error("{0}")]
    Message(String),

    /// Tokio channel send error.
    #[error("tokio channel send error: {0}")]
    TokioSendError(String),

    /// Errors from the store.
    #[error("store error: {0}")]
    StoreError(#[from] anyhow::Error),
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ZeroraftError {
    /// Creates a new `Err` result.
    pub fn custom(msg: impl Display) -> ZeroraftError {
        ZeroraftError::Message(msg.to_string())
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for ZeroraftError {
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
