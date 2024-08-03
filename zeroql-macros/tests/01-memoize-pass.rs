use std::num::NonZeroUsize;

use lru::LruCache;
use zeroql_macros::{anykey::AnyKey, memoize};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

struct RandomComputer {
    cache: LruCache<Box<dyn AnyKey>, usize>,
}

struct RandomStateComputer {
    cache: LruCache<Box<dyn AnyKey>, (usize, usize)>,
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

    let mut computer = RandomStateComputer::new();

    let value_1 = plus_rand_state(&mut computer, 2); // before call, state (count) is 0, after call, state (count) is 1
    assert_eq!(computer.count, 1);

    let value_2 = plus_rand_state(&mut computer, 3); // before call, state (count) is 1, after call, state (count) is 0
    assert_eq!(computer.count, 2);

    computer.count = 0; // Set state (count) back to 0
    assert_eq!(plus_rand_state(&mut computer, 2), value_1); // We get the cached value at state (count) = 0
    assert_eq!(computer.count, 1); // State should be updated as well

    assert_eq!(plus_rand_state(&mut computer, 3), value_2); // We get the cached value at state (count) = 1
    assert_eq!(computer.count, 2); // State should be updated as well
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[memoize(cache = self.cache)]
impl RandomComputer {
    fn new() -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(10).unwrap()),
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

impl RandomStateComputer {
    fn new() -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(10).unwrap()),
            count: 0,
        }
    }
}

#[memoize(cache = c.cache, state = c.count)]
fn plus_rand_state(c: &mut RandomStateComputer, x: usize) -> usize {
    let value = x + rand::random::<usize>();
    c.count += 1;
    value
}
