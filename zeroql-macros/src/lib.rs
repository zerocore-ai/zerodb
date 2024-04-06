//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub mod anykey;
pub mod cache;

//--------------------------------------------------------------------------------------------------
// Re-exports
//--------------------------------------------------------------------------------------------------

pub use zeroql_macros_core::{backtrack, memoize};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #[test]
    fn test_memoize() {
        let cases = trybuild::TestCases::new();

        cases.pass("tests/01-memoize-pass.rs");
        cases.pass("tests/02-backtrack-pass.rs");
        cases.pass("tests/03-memoize-and-backtrack-pass.rs");
    }
}
