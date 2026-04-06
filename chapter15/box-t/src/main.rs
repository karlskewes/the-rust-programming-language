fn main() {
    store_data_on_heap();
    cons_list(); // recursive type
}

// box_heap allocates i32 on heap and the Box on stack.
// both deallocated when go out of scope (end of function).
fn store_data_on_heap() {
    let b = Box::new(5);
    println!("b = {b}");
}

// Rust calculates allocation size required for an Enum by looking at largest variant of the Enum.
// Thus Enum variants need to be 'bounded'.
// The Nil variant below will be allocated the same space as the Cons variant, so sometimes it could
// be worth splitting Enums if there's a performance impacting size difference between variants. Rare perhaps.
#[allow(unused)]
enum List {
    // List has indefinite size
    // Cons(i32, List), // insert some indirection (e.g., a `Box`, `Rc`, or `&` to break the
    // cycle
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::mem::{align_of, align_of_val, size_of, size_of_val};
fn cons_list() {
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let c = List::Cons(1, Box::new(Nil));
    let n = List::Nil;

    println!(
        "sizes, type: {}, c: {}, n: {}",
        size_of::<List>(),
        size_of_val(&c),
        size_of_val(&n)
    );
    // sizes, type: 16, c: 16, n: 16
    // all ended up potentially padded but aligned to byte boundary.

    println!(
        "align, type: {}, c: {}, n: {}",
        align_of::<List>(),
        align_of_val(&c),
        align_of_val(&n)
    );
    // align, type: 8, c: 8, n: 8

    let enum_size = size_of::<List>();
    let largest_payload = size_of::<(i32, Box<List>)>();
    let smallest_payload = size_of::<()>(); // not same as Cons::Nil -> size 4.

    println!("enum size: {}", enum_size);
    println!("largest payload: {}", largest_payload);
    println!("smallest payload: {}", smallest_payload);
    println!("overhead (tag + padding): {}", enum_size - largest_payload);
}
