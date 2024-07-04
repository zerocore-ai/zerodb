//! Error types of the zerodb crate.

use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A specialized `Result` type for zeroql crate.
pub type Result<T> = std::result::Result<T, ZeroqlError>;

/// The main error type of the zeroql crate.
#[derive(Debug, Error)]
pub enum ZeroqlError {}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates a new `Ok` result.
#[allow(non_snake_case)]
pub fn Ok<T>(value: T) -> Result<T> {
    Result::Ok(value)
}
