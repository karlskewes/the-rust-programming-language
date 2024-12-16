// from Chapter2, bring external crate (defined in Cargo.toml) into scope.
use rand::Rng;

// std library shipped with Rust so don't need to change Cargo.toml.
use std::collections::HashMap;

// Nested paths to clean up large use lists
// use std::cmp::Ordering;
// use std::collections;
// Alternative:
use std::{cmp::Ordering, collections};

// Can use nested paths at any level, `self` keyword can be helpful.
// use std::io;
// use std::io::Write;
// Alternative:
use std::io::{self, Write};

// Glob Operator `*`
// CAUTION: can be difficult to track where name was defined.
// Often used when testing to bring everything under test into the `tests` module...
use std::collections::*;

fn main() {
    let _secret_number = rand::thread_rng().gen_range(1..=100);
}
