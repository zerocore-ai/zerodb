use std::{
    any::Any,
    hash::{DefaultHasher, Hash, Hasher},
};

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// A trait for supporting hashable keys that can be of any type.
///
/// This trait is meant to be used by hash-based data structures like `HashMap` and `HashSet`
/// that need to support keys of different types.
///
/// # Example
///
/// ```no_run
/// use zeroql_macros::anykey::AnyKey;
/// use std::collections::HashMap;
///
/// let map = HashMap::<Box<dyn AnyKey>, i32>::new();
/// map.insert(Box::new(1), 1);
/// map.insert(Box::new("1"), 1);
///
/// assert_eq!(map.get(&Box::new(1)), Some(&1));
/// assert_eq!(map.get(&Box::new("1")), Some(&1));
/// ```
pub trait AnyKey {
    /// Compares the key with another key.
    fn eq(&self, other: &dyn AnyKey) -> bool;

    /// Returns the hash of the key.
    fn hash(&self) -> u64;

    /// Returns the key as an `Any` reference.
    fn as_any(&self) -> &dyn Any;
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<T> AnyKey for T
where
    T: Hash + Eq + Clone + 'static,
{
    fn eq(&self, other: &dyn AnyKey) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            self == other
        } else {
            false
        }
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PartialEq for Box<dyn AnyKey> {
    fn eq(&self, other: &Self) -> bool {
        AnyKey::eq(self.as_ref(), other.as_ref())
    }
}

impl Eq for Box<dyn AnyKey> {}

impl Hash for Box<dyn AnyKey> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(AnyKey::hash(self.as_ref()));
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub fn into_key(key: impl Eq + Hash + Clone + 'static) -> Box<dyn AnyKey> {
    Box::new(key)
}
