use std::num::NonZeroUsize;

use lru::LruCache;
use zeroql_macros::{anykey::AnyKey, memoize};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

struct RandomComputer {
    cache: LruCache<Box<dyn AnyKey>, usize>,
    count: usize,
}

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() {
    let mut computer = RandomComputer::new();

    let value = computer.plus_rand(2);
    assert_eq!(computer.plus_rand(2), value);
    assert_eq!(computer.plus_rand(2), value);

    let value = computer.plus_rand(3);
    assert_eq!(computer.plus_rand(3), value);
    assert_eq!(computer.plus_rand(3), value);

    let value = modulo_rand(&mut computer, 2);
    assert_eq!(modulo_rand(&mut computer, 2), value);
    assert_eq!(modulo_rand(&mut computer, 2), value);

    let value_1 = plus_rand_salt(&mut computer, 2); // count goes from 0 to 1
    let value_2 = plus_rand_salt(&mut computer, 2); // count goes from 1 to 2

    assert_ne!(value_1, value_2);

    computer.count = 1; // Set count back to 0
    assert_eq!(plus_rand_salt(&mut computer, 2), value_2);

    computer.count = 0; // Set count back to 1
    assert_eq!(plus_rand_salt(&mut computer, 2), value_1);
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[memoize(cache = self.cache)]
impl RandomComputer {
    fn new() -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(10).unwrap()),
            count: 0,
        }
    }

    #[memoize]
    fn plus_rand(&mut self, x: usize) -> usize {
        x + rand::random::<usize>()
    }
}

#[memoize(cache = _c.cache)]
fn modulo_rand(_c: &mut RandomComputer, x: usize) -> usize {
    x % rand::random::<usize>()
}

#[memoize(cache = c.cache, salt = c.count)]
fn plus_rand_salt(c: &mut RandomComputer, x: usize) -> usize {
    c.count += 1;
    x + rand::random::<usize>()
}
