use std::num::NonZeroUsize;

use lru::LruCache;
use zeroql_macros::{memoize, Cache};

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() {
    let mut test = Test {
        cache: LruCache::new(NonZeroUsize::new(10).unwrap()),
        cursor: 0,
    };

    let value = test.check_even(2);
    assert_eq!(test.check_even(2), value);
    assert_eq!(test.check_even(2), value);
}

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

struct Test {
    cache: LruCache<[u8; 32], Option<u32>>,
    cursor: u32,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Test {
    #[memoize(cache = self.cache, key_extension = self.cursor)]
    fn check_even(&mut self, x: u32) -> Option<u32> {
        // // Eagerly set cursor
        // self.cursor += 1;

        if x % 2 == 0 {
            return Some(x + rand::random::<u32>());
        }

        None
    }
}
