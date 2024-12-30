use std::net::IpAddr;

pub struct Guess {
    value: i32, // private, must use constructor `new()`.
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // enforce constraint on value.
        // Should this panic or return an error though? Apparently to be discussed
        // in Chapter 14.
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    // getter because `value` is private to prevent setting outside of our
    // constructor enforced rules (1<= value <= 100)
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    // Examples, Prototypes, Tests

    // 1. robust error handling can obscure interesting code trying to show.
    // 2. `unwrap` and `expect` can be used as markers to revisit (TODO).
    // 3. abort test if setup dependency or function not under test fails.

    // Cases in which you have more information than the compiler

    // parse returns a Result
    // expect returns value or panics
    // Remove Result handling work for caller by using expect() or unwrap() on
    // Result's that should never panic.
    let _home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}
