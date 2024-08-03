//! Combinator module for zeroql compiler.

mod combinator;
#[cfg(test)]
mod tests;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use combinator::*;

//--------------------------------------------------------------------------------------------------
// Working
//--------------------------------------------------------------------------------------------------

// use std::collections::HashMap;

// use crate::ast::Ast;

// use super::{Parser, ParserResult};

// fn permutation_init<'a>(
//     permutation: Vec<String>,
//     parents: Vec<String>,
//     map: HashMap<String, Box<dyn Fn(&mut Parser) -> ParserResult<Option<Ast<'a>>>>>,
// ) {
//     for parent in parents {
//     }

//     todo!()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_permutation() {
//         permutation_init();
//     }
// }
