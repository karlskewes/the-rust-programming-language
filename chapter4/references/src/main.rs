fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let s = String::from("hello");
    change_errors(&s); // immutable reference by default, like variables without 'mut'

    let mut s = String::from("hello");
    change(&mut s);
    println!("s: {s}");

    let mut s = String::from("hello");

    let _r1 = &mut s;
    // ERRORS: Can't have multiple mixed immutable & mutable borrows (references) to a value.
    // let r2 = &mut s; // ERROR: second mutable borrow occurs here
    //
    // println!("r1: {r1}, r2: {r2}");

    // let r3 = &s; // can't borrow 's' as immutable because also borrowed as mutable.
    // println!("r1: {r1}, r3: {r3}");

    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so can make a new reference

    let _r2 = &mut s;

    // can have multiple immutable references to a value
    let mut s = String::from("hello"); // warning: variable does not need to be mutable

    let r1 = &s; // immutable ok
    let r2 = &s; // immutable ok
                 // let r3 = &mut s; // mutable NOT OK
    let r3 = &s;

    println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("r1: {r1}, r2: {r2}");

    let r3 = &mut s; // okay, immutable r1 and r2 are no longer being used
    println!("r3: {r3}");

    // dangling references prevented by compiler.

    // let reference_to_nothing = dangle(); // ERROR

    // have to return value directly
    let s = no_dangle();
    println!("s: {s}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_errors(_some_string: &String) {
    // some_string.push_str(", world");  // ERROR: reference value, can't  borrow as mutable.
}

// mut keyword required in parameters
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// dangle() ERROR: returns a borrowed value, but there is no value for it to be borrowed from.
// fn dangle() -> &String {
//     let s = String::from("hello"); // 's' goes out of scope when function ends and deallocated.
//
//     &s // attempt to return value that will be deallocated, ERROR.
// }

fn no_dangle() -> String {
    String::from("hello") // or let s = .... ; s
}
