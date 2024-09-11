fn main() {
    // numbers: integers and floats
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    println!("add two floats: {}", x + y);

    // additions
    let sum = 5 + 10;
    println!("sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product: {product}");

    // division
    let quotient = 56.7 / 32.3;
    let truncated = -5 / 3; // Results in -1
    println!("quotient: {quotient}");
    println!("truncated: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");

    // bool
    let t = true;
    let f: bool = false;
    println!("t: {t} f: {f}");

    // char
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("c: {c} z: {z} heart_eyed_cat: {heart_eyed_cat}");

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup:#? {tup:#?}, tup:? {tup:?}"); // pretty-print both ways.

    // destructure by assigning to each element
    let (_x, y, _z) = tup; // underscore prefix to ignore unused warning.
    println!("The value of y is: {y}");

    // index into tuple
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred: {five_hundred} six_point_four: {six_point_four} one: {one}");

    // arrays - fixed size
    let a = [1, 2, 3, 4, 5];
    println!("a: {a:?} or {a:#?}");

    // good when fixed number of elements
    // const requires type notation which for an array includes size.
    const _MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {a:?}");
    let a = [3; 5];
    println!("a initialized to same value: {a:?}");

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first: {first} second: {second}");

    // out of bounds array access example:
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is {element}");

    /*
    Please enter an array index.
    2
    The value of the element at index 2 is 3
    */

    /*
    Please enter an array index.
    6
    thread 'main' panicked at src/main.rs:101:19:
    index out of bounds: the len is 5 but the index is 6
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */

    /* $ RUST_BACKTRACE=1 cargo run
    Please enter an array index.
    6
    thread 'main' panicked at src/main.rs:101:19:
    index out of bounds: the len is 5 but the index is 6
    stack backtrace:
       0: rust_begin_unwind
       1: core::panicking::panic_fmt
       2: core::panicking::panic_bounds_check
       3: data_types::main
                 at ./src/main.rs:101:19
       4: core::ops::function::FnOnce::call_once
                 at /build/rustc-1.78.0-src/library/core/src/ops/function.rs:250:58
    */

    // Functions

    another_function();
    another_function2(2);
    print_labeled_measurement(5, 'h');

    // Statements & Expressions

    let _y = 6;
    // let x = (let y = 6); // invalid, let is a statement, doesn't return value for x to bind to.

    let y = {
        let x = 3; // semi-colon, designating a statement.
        x + 1 // no semi-colon, this is an expression and ultimately is the value bound to y.
    };

    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");

    // Comments

    // Double slash per line
    // Multi line idiom is for each use line to be preceeded with double slash

    // Control flow

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // ERROR: no automatic conversion to bool like other Javascript, etc.
    // if number {
    //     println!("number was three");
    // }

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3"); // this
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // ^^ too much branching, use a `match` instead..

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}"); // 5

    // let number = if condition { 5 } else { "six" }; // mismatched types. Single type only.

    // Loops

    loop {
        println!("again!");

        break; // avoid iterating indefinitely
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // expression after break evaluated
        }

        // continue; // implicit
    };

    println!("The results is {result}");

    let mut count = 0;

    // labelled loop, must begin with single quote `'`.
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // inner loop only
            }
            if count == 2 {
                break 'counting_up; // break outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // error prone, array size changed without changing loop condition.
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    // idiomatic, loop items in array
    for element in a {
        println!("the value is: {element}");
    }

    // idiomatic, use standard libraries Range (start..end)
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // x + 1 // implicit return (expression)
    // x + 1; // error, statement not expression. "mismatched types"
    return x + 1; // explicit return statement.
}
