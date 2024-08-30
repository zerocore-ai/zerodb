//! Semantic analysis for the ZeroQL language.
//!
//! This module contains the semantic analysis for the ZeroQL language. It is responsible for
//! the following:
//! - Type checking
//! - Name resolution
//! - Constant folding
//! - Constant propagation
//! - Dead code elimination
//! - ...and more

mod error;
mod pass;
mod schema;
mod sema;
mod symbols;
mod traits;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use error::*;
pub use pass::*;
pub use schema::*;
pub use sema::*;
pub use symbols::*;
