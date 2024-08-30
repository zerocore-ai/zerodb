use thiserror::Error;
use zeroutils_path::Path;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The result of a semantic analysis operation.
pub type SemaResult<T> = Result<T, SemaError>;

/// An error that occurred during semantic analysis.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum SemaError {
    /// An unexpected AST kind.
    #[error("unexpected ast kind: {0}")]
    UnexpectedAstKind(String),

    /// An undefined variable.
    #[error("undefined variable or parameter: {0}")]
    UndefinedVariableOrParam(String),

    /// An undefined schema item.
    #[error("undefined schema item: {0}")]
    UndefinedSchemaItem(Path),

    /// An undefined database.
    #[error("undefined database: {0}")]
    UndefinedDatabase(Path),

    /// An unspecified database.
    #[error("unspecified database")]
    UnspecifiedDatabase,

    /// A duplicate table or edge definition.
    #[error("duplicate schema item definition: {0}")]
    DuplicateSchemaItemDefinition(Path),

    /// A duplicate type or enum definition.
    #[error("duplicate type or enum definition: {0}")]
    DuplicateTypeOrEnumDefinition(String),

    /// A path error.
    #[error("path error: {0}")]
    PathError(#[from] zeroutils_path::PathError),
}
