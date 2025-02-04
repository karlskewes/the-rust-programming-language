// Files in the ../tests/ directory will be compiled individually as crates.
use adder::add_two; // import public functions that wish to test.

mod common; // include test helper

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}

// Run single integration test with:
// `cargo test --test <test_file_excluding_extension>

// NOTE: Binary testing
// we can't create integration tests in the `tests` directory and bring functions
// defined in ../src/main.rs into scope with a `use` statement.
// This is one of the reasons why projects that provide a binary have a
// straightforward src/main.rs file that calls logic in src/lib.rs file.
