fn main() {
    let _s = "hello";

    {
        let s = "hello";
        println!("s: {s}");
    }

    // println!("s: {s}"); // cannot find value `s` in this scope

    let s = String::from("hello");
    drop(s); // deallocation

    let mut s = String::from("hello");

    s.push_str(", world!"); // append literal to String

    println!("{s}");

    {
        let s = String::from("hello"); // s valid from now
        println!("scope: {s}");
    } // scope over, s no longer valid `drop(s)`

    println!("{s}"); // hello, world!

    // deep copy of integer types (and others - simple scalar values) by storing both on stack.

    let x = 5;
    let _y = x; // two variables, different memory.

    // `move`, not `shallow copy`

    let s1 = String::from("hello");
    let _s2 = s1; // s2 contains ptr,len,cap, with ptr to same backing array as s1. Go Slice like.
                  // `move` = s1 was "moved" into s2. s1 invalidated.

    // println!("{s1}, world!"); // borrow of moved value: `s1`, value borrowed here after move

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // stack allocated deep copy again - simple scalar type implements `Copy` trait.
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // Ownership and Functions

    let s = String::from("hello");
    takes_ownership(s); // s `moved` to function, no longer valid in main().

    let x = 5;
    makes_copy(x); // i32 is Copy.

    // println!("s: {s}"); // s is borrowed and not "returned" as such.

    println!("x: {x}"); // x still in scope.

    // Return Values and Scope
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("s1: {s1}");
    // println!("s2: {s2}"); // s2 borrowed, no longer in scope here.
    println!("s3: {s3}");

    // returning ownership of parameters

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); // return s1 as s2 to "give back ownership"
    println!("The length of '{s2}' is {len}.");
}

fn takes_ownership(some_string: String) {
    println!("some_string: {some_string}");
} // `some_string` goes out of scope and `drop` is called, backing memory freed.

fn makes_copy(some_integer: i32) {
    println!("some_integer: {some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string // return
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length) // return a tuple
}
