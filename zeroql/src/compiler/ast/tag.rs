use crate::sema::{Symbols, VersionedSchema};
//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Semantic analysis information that can be associated with an AST node.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnalysisTag {
    symbols: Option<Symbols>,
    schema: Option<VersionedSchema>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl AnalysisTag {
    /// Sets the symbols for the analysis tag.
    pub fn set_symbols(&mut self, symbols: Symbols) {
        self.symbols = Some(symbols);
    }

    /// Sets the schema for the analysis tag.
    pub fn set_schema(&mut self, schema: VersionedSchema) {
        self.schema = Some(schema);
    }

    /// Gets the symbols for the analysis tag.
    pub fn get_symbols(&self) -> Option<&Symbols> {
        self.symbols.as_ref()
    }

    /// Gets the schema for the analysis tag.
    pub fn get_schema(&self) -> Option<&VersionedSchema> {
        self.schema.as_ref()
    }
}
