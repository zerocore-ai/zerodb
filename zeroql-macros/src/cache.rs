use std::collections::HashMap;

use lru::LruCache;

use crate::anykey::AnyKey;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// A trait for supporting caches that can be used by the [`memoize`][zeroql_macros::memoize] attribute macro.
///
/// Types that implement this trait are usually key-value data structures that can be used to cache the
/// result of a function call. In fact, this trait is already implemented for `HashMap` and `LruCache`.
///
/// # Example
///
/// ```no_run
/// use zeroql_macros::cache::Cache;
///
/// struct MyCache {
///    cache: HashMap<Box<dyn AnyKey>, i32>,
/// }
///
/// impl Cache for MyCache {
///     type Value = i32;
///
///     fn get(&mut self, key: &Box<dyn AnyKey>) -> Option<&Self::Value> {
///         self.cache.get(key)
///     }
///
///     fn insert(&mut self, key: Box<dyn AnyKey>, value: Self::Value) -> Option<Self::Value> {
///         self.cache.insert(key, value)
///     }
/// }
/// ```
pub trait Cache {
    /// The type of the value stored in the cache.
    type Value;

    /// Gets the value associated with the key.
    #[allow(clippy::borrowed_box)]
    fn get(&mut self, key: &Box<dyn AnyKey>) -> Option<&Self::Value>;

    /// Inserts a key-value pair into the cache.
    fn insert(&mut self, key: Box<dyn AnyKey>, value: Self::Value) -> Option<Self::Value>;
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<V> Cache for HashMap<Box<dyn AnyKey>, V> {
    type Value = V;

    fn get(&mut self, key: &Box<dyn AnyKey>) -> Option<&Self::Value> {
        <HashMap<Box<dyn AnyKey>, V>>::get(self, key)
    }

    fn insert(&mut self, key: Box<dyn AnyKey>, value: Self::Value) -> Option<Self::Value> {
        <HashMap<Box<dyn AnyKey>, V>>::insert(self, key, value)
    }
}

impl<V> Cache for LruCache<Box<dyn AnyKey>, V> {
    type Value = V;

    fn get(&mut self, key: &Box<dyn AnyKey>) -> Option<&Self::Value> {
        <LruCache<Box<dyn AnyKey>, V>>::get(self, key)
    }

    fn insert(&mut self, key: Box<dyn AnyKey>, value: Self::Value) -> Option<Self::Value> {
        <LruCache<Box<dyn AnyKey>, V>>::put(self, key, value)
    }
}
