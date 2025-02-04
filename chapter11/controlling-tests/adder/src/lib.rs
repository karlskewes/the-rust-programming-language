fn print_and_returns10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

#[cfg(test)]
// can use any module name to group tests.
// EDIT: convention is for module named `tests` in each file. See ../../../organizing-tests/
mod test_output_printing {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = print_and_returns10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = print_and_returns10(8);
        assert_eq!(value, 5);
    }
    /*
    ---- test_output_printing::this_test_will_fail stdout ----
    I got the value 8
    thread 'tests::this_test_will_fail' panicked at src/lib.rs:19:9:
    assertion `left == right` failed
      left: 10
     right: 5
    */

    // Print output regardless of success or failure.

    // On test failure any stdout captured will be printed.
    // On test success any stdout captured will not be printed.
    // Unless run:
    // $ cargo test -- --show-output
    /*
        ---- tests::this_test_will_pass stdout ----
    I got the value 4

    successes:
        test_output_printing::this_test_will_pass
    */
}

#[cfg(test)]
// can use any module name to group tests.
mod test_selective_run {
    use super::*;

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }

    // Run a single test with `cargo test <test_name>`

    /*
    $ cargo test one_hundred
        Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
         Running unittests src/lib.rs (target/debug/deps/adder-26281bcaa06a4ef4)

    running 1 test
    test test_selective_run::one_hundred ... ok
    */

    // Run tests for a single module with `cargo test <module_name>`

    // Run tests for a single test in module with
    // `cargo test <module_name>::<test_name_or_prefix>`
}

#[cfg(test)]
mod test_ignores {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    // Ignoring specific tests by default, perhaps because long running.

    /*
    $ cargo test test_ignores
        Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
         Running unittests src/lib.rs (target/debug/deps/adder-26281bcaa06a4ef4)

    running 2 tests
    test test_ignores::expensive_test ... ignored
    test test_ignores::it_works ... ok

    test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 5 filtered out; finished in 0.00s
    */

    // Electing to run ignored tests with `cargo test -- --ignored`

    /*
    $ cargo test test_ignores -- --ignored
        Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
         Running unittests src/lib.rs (target/debug/deps/adder-26281bcaa06a4ef4)

    running 1 test
    test test_ignores::expensive_test ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s
    */

    // Or all tests with `cargo test -- --include-ignored`
    /*
    $ cargo test test_ignores -- --include-ignored
        Compiling adder v0.1.0 (/home/karl/src/github.com/karlskewes/the-rust-programming-language/chapter11/controlling-tests/adder)
         Finished `test` profile [unoptimized + debuginfo] target(s) in 0.14s
          Running unittests src/lib.rs (target/debug/deps/adder-26281bcaa06a4ef4)

    running 2 tests
    test test_ignores::expensive_test ... ok
    test test_ignores::it_works ... ok

    test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s
    */

    // QUESTION: Can we have tags and select by tags? e.g: unit, integration, etc
    // Perhaps using a filter and convention in test name, e.g: xyz_integration?
    // EDIT: See ../../../organizing-tests/adder/tests/ for integration tests
    // and convention details.
    // $ cargo test -- --skip <FILTER>
    //         --skip FILTER   Skip tests whose names contain FILTER (this flag can
    //                         be used multiple times)

    // Example skipping below, keyword `works`, could have been `integration` or convention.
    /*
    $ cargo test test_ignores -- --skip works
        Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
         Running unittests src/lib.rs (target/debug/deps/adder-26281bcaa06a4ef4)

    running 1 test
    test test_ignores::expensive_test ... ignored

    test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 6 filtered out; finished in 0.00s
    */
}
