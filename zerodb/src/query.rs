use serde::{Deserialize, Serialize};
use zeroraft::{Request, Response};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// TODO(appcypher): To be replaced with the right command variants.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Query {
    /// Delete a key.
    Delete(String),
    /// Set a key to a value.
    Set(String, String),
    /// Get a key.
    Get(String),
}

/// TODO(appcypher): To be replaced with the right command variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryResponse {}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Request for Query {}

impl Response for QueryResponse {}
