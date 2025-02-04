#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be great than or equal to 1, got {value}");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}");
        }

        // With `[should_panic]` the reason can be imprecise and output too.
        // ---- tests::greater_than_100 stdout ----
        // note: test did not panic as expected
        // if value < 1
        // /*|| value > 100*/
        // {
        //     panic!("Guess value must be between 1 and 100, got {value}.");
        // }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // tests is an 'inner' module, bring methods from 'outer' module in (under test).
    fn _some_helper() {
        println!("helper func that isn't a test and so won't be run by `cargo test`");
    }

    // annotate test functions, but don't annotate helper functions to signal what is and isn't a test
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        // NOTE: Params are called 'left' and 'right', not 'expected' and 'actual'
        assert_eq!(result, 4);

        // If change function:
        /*
        thread 'tests::it_adds_two' panicked at src/lib.rs:71:9:
        assertion `left == right` failed
          left: 23
          right: 4

         */
    }

    #[test]
    fn cant_assert_struct_equality_without_trait_implementation() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // error[E0369]: binary operation `==` cannot be applied to type `Rectangle`
        // note: an implementation of `PartialEq` might be missing for `Rectangle`
        // Can use `#[derive(PartialEq, Debug)]` annotation.
        assert_ne!(larger, smaller);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`",
        );
    }

    #[test]
    // #[should_panic(expected = "greater than or equal to 100")]
    // ---- tests::greater_than_100 stdout ----
    // thread 'tests::greater_than_100' panicked at src/lib.rs:34:13:
    // Guess value must be less than or equal to 100, got 200
    // note: panic did not contain expected string
    //       panic message: `"Guess value must be less than or equal to 100, got 200"`,
    //  expected substring: `"greater than or equal to 100"`
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Using Result<T, E> In Tests
    // This enables using the ? operator in the body of tests, which can be a convenient way to
    // write tests that should fail if any operation within them returns an Err.

    #[test]
    fn it_works_return_err() -> Result<(), String> {
        let result = add(2, 2);

        // if result == 5 {
        // ---- tests::it_works_return_err stdout ----
        // Error: "two plus two does not equal four"
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    // #[should_panic] // can't use on on fn returning Result<T, E>
    fn it_works_is_err() {
        fn return_result_err() -> Result<(), String> {
            if true {
                Err(String::from("generated error"))
            } else {
                Ok(())
            }
        }

        let result = return_result_err();

        assert!(result.is_err())
    }
}
