use crate::ast::Ast;

use super::{NameResolver, SemaResult, TypeInferencer};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The semantic analyzer.
pub struct SemanticAnalyzer<'a> {
    /// The AST to analyze.
    ast: &'a mut Ast<'a>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<'a> SemanticAnalyzer<'a> {
    /// Creates a new semantic analyzer.
    pub fn new(ast: &'a mut Ast<'a>) -> Self {
        Self { ast }
    }

    /// Analyzes the AST.
    pub fn analyze(&mut self) -> SemaResult<()> {
        NameResolver::new().analyze(self.ast)?;
        TypeInferencer::new().analyze(self.ast)?;
        Ok(())
    }

    /// Gets the AST.
    pub fn get_ast(&self) -> &Ast<'a> {
        self.ast
    }
}
