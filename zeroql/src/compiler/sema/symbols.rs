use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `Symbols` represents the variables introduced within a given scope, like in `LET` or `FOR` expressions.
/// Scopes are typically introduced by constructs like `FOR`, `WHILE`, `FUNCTION`, and `IF`.
///
/// ```txt
/// FOR $x IN y {
///     LET $z = $x + 1;
/// }
/// ```
///
/// In this code, `$x` is a symbol introduced by `FOR`, and `$z` is introduced by `LET`.
#[derive(Debug, PartialEq, Default)]
pub struct Symbols {
    inner: Arc<SymbolsInner>,
}

/// Internal representation for storing symbol metadata.
#[derive(Debug, Default)]
pub struct SymbolsInner {
    /// The table of symbols.
    table: Mutex<HashMap<String, SymbolMeta>>,

    /// The parent symbol in an outer scope.
    parent: Option<Symbols>,
}

/// Metadata associated with a symbol.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SymbolMeta {}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Symbols {
    /// Creates a new symbol table.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(SymbolsInner {
                table: Mutex::new(HashMap::new()),
                parent: None,
            }),
        }
    }

    /// Creates a new symbol table with a parent.
    pub fn with_parent(parent: &Symbols) -> Self {
        Self {
            inner: Arc::new(SymbolsInner {
                table: Mutex::new(HashMap::new()),
                parent: Some(parent.clone()),
            }),
        }
    }

    /// Inserts a new symbol into the table with the given metadata.
    pub fn insert(&self, name: String, meta: SymbolMeta) {
        self.inner.table.lock().unwrap().insert(name, meta);
    }

    /// Checks if a symbol exists in the current table or any parent table.
    pub fn contains(&self, name: &str) -> bool {
        let table = self.inner.table.lock().unwrap();
        if table.contains_key(name) {
            return true;
        }

        self.inner
            .parent
            .as_ref()
            .map_or(false, |parent| parent.contains(name))
    }

    /// Gets a symbol metadata from the table and calls the provided closure with it.
    pub fn get_with<T>(&self, name: String, f: impl FnOnce(Option<&SymbolMeta>) -> T) -> T {
        let table = self.inner.table.lock().unwrap();
        let meta = table.get(&name);
        f(meta)
    }

    /// Gets a mutable symbol metadata from the table and calls the provided closure with it.
    pub fn get_mut_with<T>(&self, name: String, f: impl FnOnce(Option<&mut SymbolMeta>) -> T) -> T {
        let mut table = self.inner.table.lock().unwrap();
        let meta = table.get_mut(&name);
        f(meta)
    }

    /// Retrieves the parent symbol (in an outer scope) if it exists.
    pub fn parent(&self) -> Option<&Symbols> {
        self.inner.parent.as_ref()
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Clone for Symbols {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl PartialEq for SymbolsInner {
    fn eq(&self, other: &Self) -> bool {
        (std::ptr::eq(&self.table, &other.table)
            || *self.table.lock().unwrap() == *other.table.lock().unwrap())
            && self.parent.eq(&other.parent)
    }
}
