use std::{thread, time::Duration};

#[derive(Debug, Clone, Copy)] // Copy values automatically instead of requiring .clone()
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
        // || => capture error,
        // nothing to capture here.
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    // Traits:
    // FnOnce -> implemented by all closures, because all closures can be called at
    // least once. Closure may move captured values out of body.
    // FnMut -> closures that don't move captured values out of their body but might
    // mutate captured values. Can be called more than once.
    // Fn -> Closures that don't move captured values and don't mutate captured values,
    // can be called multiple times, even concurrently.

    // Stdlib unwrap_or_else
    // FnOnce closure, moves captured value T out so can only be called once.
    {
        /*
        impl<T> Option<T> {
            pub fn unwrap_or_else<F>(self, f: F) -> T
            where
                F: FnOnce() -> T, // note FnOnce which all closures implement
            {
                match self {
                    Some(x) => x,
                    None => f(),
                }
            }
        }
        */
    }

    closure_type_inference_and_annotation();
    capturing_environment_with_closures();
    capturing_references_or_moving_ownership();
    fnmut_multiple_calls();
    fnonce_multiple_calls();
}

// Closure Type Inference and Annotation
fn closure_type_inference_and_annotation() {
    {
        // types are normally inferable for closures but we can still annotate.
        // closures are not normally exposed to end users either so less typing
        // is fine.
        // Additionally can assign closures to variables (like Go).
        let expensive_closure = |num: u32| -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };
    }
    {
        // contrast closure syntax with function syntax.
        // similar except use of pipes | with closures
        const fn _add_one_v1(x: u32) -> u32 {
            x + 1
        }
        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        let add_one_v3 = |x| {
            x + 1 // type annotations needed until use
        };
        let add_one_v4 = |x| x + 1; // braces optional where simple

        let _r3 = add_one_v3(10); // required in order for type inference of closure
        let _r4 = add_one_v4(5); // required in order for type inference of closure
    }
    {
        // first caller will lock in the types
        let example_closure = |x| x;

        let _s = example_closure(String::from("hello"));
        // let _n = example_closure(5); // mismatched types String and integer
    }
}

// Capturing the Environment with Closures
fn capturing_environment_with_closures() {
    let store: Inventory = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {user_pref1:?} gets {giveaway1:?}");

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {user_pref2:?} gets {giveaway2:?}");
}

// Capturing References or Moving Ownership
fn capturing_references_or_moving_ownership() {
    {
        let list = vec![1, 2, 3];
        println!("Before dining closure: {list:?}");

        let only_borrows = || println!("From closure: {list:?}");

        println!("Before calling closure: {list:?}");
        only_borrows(); // Fn ??

        println!("After calling closure: {list:?}");
    }
    {
        let mut list = vec![1, 2, 3];
        println!("Before defining losure: {list:?}");

        let mut borrows_mutably = || list.push(7);
        // println!("Before calling closure: {list:?}"); // cannot borrow 'list' as
        // immutable...

        borrows_mutably(); // FnMut
        borrows_mutably(); // FnMut
        println!("After calling closure: {list:?}");
    }
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        // `move` keyword required as transfer ownership of list into closure/thread.
        thread::spawn(move || println!("From thread: {list:?}")) // FnOnce
            .join()
            .unwrap();

        // println!("After thread: {list:?}"); // borrow of moved value: list
    }
}

// Stdlib sort_by_key
// FnMut closure, calls closure multiple times one for each item in slice.
fn fnmut_multiple_calls() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        _height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            _height: 1,
        },
        Rectangle {
            width: 3,
            _height: 5,
        },
        Rectangle {
            width: 7,
            _height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    // list.sort_by_key(|r| r._height);
    println!("{list:#?}");
}

// Stdlib sort_by_key
// FnOnce closure attempt to call multiple times -> error
fn fnonce_multiple_calls() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        _height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            _height: 1,
        },
        Rectangle {
            width: 3,
            _height: 5,
        },
        Rectangle {
            width: 7,
            _height: 12,
        },
    ];

    let mut sort_operations = vec![];
    let mut num_sort_operations = 0;
    let value = String::from("closure called"); // captured outer variable

    // captured by this `FnMut` closure
    list.sort_by_key(|r| {
        /*
        1. cannot move out of `value`, a captured variable in an `FnMut` closure
           move occurs because `value` has type `String`, which does not implement the `Copy` trait [E0507]
        2. consider cloning the value if the performance cost is acceptable: `.clone()` [E0507]
        */
        // sort_operations.push(value);

        // Alternatives:
        sort_operations.push(value.clone());
        num_sort_operations += 1;

        r.width
    });
    // list.sort_by_key(|r| r._height);
    println!("list: {list:#?}");
    println!("sort_operations: {sort_operations:?}");
    println!("num_sort_operations: {num_sort_operations}");
}
