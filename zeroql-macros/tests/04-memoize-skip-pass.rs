use std::num::NonZeroUsize;

use lru::LruCache;
use zeroql_macros::{anykey::AnyKey, memoize};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

struct Counter {
    cache: LruCache<Box<dyn AnyKey>, (usize, usize)>,
    count: usize,
}

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() {
    // First we test the case where the skip function is not provided

    // The `increment` method should always be memoized

    let mut counter = Counter::new();
    assert_eq!(counter.cache.len(), 0);

    let _ = counter.increment();
    assert_eq!(counter.cache.len(), 1);

    let _ = counter.increment();
    assert_eq!(counter.cache.len(), 2);

    let _ = counter.increment();
    assert_eq!(counter.cache.len(), 3);

    // The `increment_skip` method should only be memoized if the skip function returns false

    let mut counter = Counter::new();
    assert_eq!(counter.cache.len(), 0);

    let value = increment_skip(&mut counter);
    assert_eq!(value, 1);
    assert_eq!(counter.cache.len(), 1);

    let value = increment_skip(&mut counter);
    assert_eq!(value, 2);
    assert_eq!(counter.cache.len(), 1);

    let value = increment_skip(&mut counter);
    assert_eq!(value, 3);
    assert_eq!(counter.cache.len(), 2);
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[memoize(cache = self.cache, state = self.count)]
impl Counter {
    fn new() -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(10).unwrap()),
            count: 0,
        }
    }

    #[memoize]
    fn increment(&mut self) -> usize {
        self.count += 1;
        self.count
    }
}

#[memoize(cache = c.cache, state = c.count, skip = |r| r % 2 == 0)]
fn increment_skip(c: &mut Counter) -> usize {
    c.count += 1;
    c.count
}
