use serde::{Deserialize, Serialize};

use crate::{Request, Response};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MockRequest {
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MockResponse {
    Empty,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Request for MockRequest {}

impl Response for MockResponse {}
