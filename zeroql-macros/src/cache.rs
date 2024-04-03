use lru::LruCache;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

pub trait Cache {
    type Value;

    fn get(&mut self, key: &[u8; 32]) -> Option<&Self::Value>;
    fn insert(&mut self, key: [u8; 32], value: Self::Value) -> Option<Self::Value>;
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<U> Cache for LruCache<[u8; 32], U> {
    type Value = U;

    fn get(&mut self, key: &[u8; 32]) -> Option<&Self::Value> {
        <LruCache<[u8; 32], U>>::get(self, key)
    }

    fn insert(&mut self, key: [u8; 32], value: Self::Value) -> Option<Self::Value> {
        <LruCache<[u8; 32], U>>::put(self, key, value)
    }
}
