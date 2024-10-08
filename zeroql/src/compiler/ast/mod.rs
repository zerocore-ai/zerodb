//! Abstract Syntax Tree (AST) for the zeroql compiler.

#[allow(clippy::module_inception)]
mod ast;
mod tag;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use ast::*;
pub use tag::*;
