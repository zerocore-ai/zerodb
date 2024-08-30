use std::sync::Arc;

use zeroutils_path::Path;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `VersionedSchema` represents an entity that can either be a database, table or field. These are
/// called schemas and their metadata can be change over time which is why they are versioned.
#[derive(Debug, PartialEq)]
pub struct VersionedSchema {
    inner: Arc<VersionedSchemaInner>,
}

/// `VersionedSchemaInner` represents the inner representation of a `VersionedSchema`.
#[derive(Debug, Clone, PartialEq)]
pub struct VersionedSchemaInner {
    name: Path,
    meta: SchemaMeta,
    previous: Option<VersionedSchema>,
}

/// `SchemaMeta` represents the metadata of a schema.
#[derive(Clone, Debug, PartialEq)]
pub enum SchemaMeta {
    /// A database schema.
    Database(),

    /// A table schema.
    Table(),

    /// An edge schema.
    Edge(),

    /// A type schema.
    Type(),

    /// An enum schema.
    Enum(),

    /// An index schema.
    Index(),

    /// A module schema.
    Module(),

    /// A param schema.
    Param(),
}

/// `DatabaseSchema` represents the schema of a database.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DatabaseSchema {}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl VersionedSchema {
    /// Creates a new `VersionedSchema`.
    pub fn new(
        name: impl Into<Path>,
        meta: impl Into<SchemaMeta>,
        previous: Option<&VersionedSchema>,
    ) -> Self {
        Self {
            inner: Arc::new(VersionedSchemaInner {
                name: name.into(),
                meta: meta.into(),
                previous: previous.cloned(),
            }),
        }
    }

    /// Creates a new `VersionedSchema` with a previous version.
    pub fn with_previous(
        name: impl Into<Path>,
        meta: impl Into<SchemaMeta>,
        previous: &VersionedSchema,
    ) -> Self {
        Self {
            inner: Arc::new(VersionedSchemaInner {
                name: name.into(),
                meta: meta.into(),
                previous: Some(previous.clone()),
            }),
        }
    }

    /// Gets the schema of a schema entity.
    pub fn get(&self, name: &Path) -> Option<&SchemaMeta> {
        let mut current = self;
        loop {
            if &current.inner.name == name {
                return Some(&current.inner.meta);
            }

            if let Some(previous) = current.inner.previous.as_ref() {
                current = previous;
            } else {
                break;
            }
        }
        None
    }

    /// Checks if a schema contains a schema entity.
    pub fn contains(&self, name: &Path) -> bool {
        self.get(name).is_some()
    }
}

impl DatabaseSchema {
    /// Gets the schema of a database entity.
    pub fn get(&self, _name: &Path) -> Option<SchemaMeta> {
        // TODO: Implement
        None
    }

    /// Checks if a schema contains a schema entity.
    pub fn contains(&self, _name: &Path) -> bool {
        // TODO: Implement
        false
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Clone for VersionedSchema {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
