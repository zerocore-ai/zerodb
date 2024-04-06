use zeroql_macros::backtrack;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

struct Counter {
    state: isize,
}

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() {
    let mut counter = Counter::new();

    assert_eq!(counter.inc_even(2), Some(2));
    assert_eq!(counter.inc_even(3), None);
    assert_eq!(counter.state, 2);

    assert_eq!(dec_odd(&mut counter, 1), Some(1));
    assert_eq!(dec_odd(&mut counter, 2), None);
    assert_eq!(counter.state, 1);
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[backtrack(state = self.state)]
impl Counter {
    fn new() -> Self {
        Self { state: 0 }
    }

    #[backtrack]
    fn inc_even(&mut self, n: isize) -> Option<isize> {
        self.state += n; // Modify the state ahead of time

        if n % 2 == 0 {
            return Some(self.state);
        }

        None
    }
}

#[backtrack(state = _c.state)]
fn dec_odd(_c: &mut Counter, n: isize) -> Option<isize> {
    _c.state -= n; // Modify the state ahead of time

    if n % 2 != 0 {
        return Some(_c.state);
    }

    None
}
