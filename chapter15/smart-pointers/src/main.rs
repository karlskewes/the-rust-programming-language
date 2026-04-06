use std::ops::Deref;

fn main() {
    follow_ref_to_value();
    use_box_t_like_ref();
    follow_ref_with_mybox();
    deref_coercion();
    deref_variations();
}

fn follow_ref_to_value() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // assert_eq!(5, y); // can't compare integer and &integer.
    assert_eq!(5, *y); // dereference y with * to assert value.
}

fn use_box_t_like_ref() {
    let mut x = 5;
    let y = Box::new(x); // y is instance of a box pointing to COPIED value of x.
    assert_eq!(5, x);
    assert_eq!(5, *y);

    x = 10;
    assert_ne!(10, *y); // x was copied and boxed for y.
    assert_eq!(5, *y);
    println!("x: {x}");
}

// Defining Our Own Smart Pointer

/// MyBox is a tuple struct with one element of type `T`.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // tuple field access
    }
}

fn follow_ref_with_mybox() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // without impl Deref: "type `MyBox<{integer}>` cannot be dereferenced."
    assert_eq!(5, *(y.deref())); // `*y` is shorthand for this. Thanks compiler team.
}

// Using Deref Coercion in Functions and Methods

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// Rust will analyze the types and use Deref::deref as many times as necessary to match the
// parameters types.
// No runtime penalty for taking advantage of deref coercion!
fn deref_coercion() {
    let m = MyBox::new(String::from("Rust"));
    // 1. &MyBox<String> -> &String via MyBox.deref()
    // 2. stdlib does String.deref() -> &str
    // 3. hello(...) takes &str
    hello(&m);

    // without deref coercion:
    // 1. (*m) deref's MyBox<String> into String (via m.deref());
    // 2. & plus [..] take a string slice (&str) from String.
    // 3. hello(...) takes a &str
    hello(&(*m)[..]); // without deref coercion.
}

// Handling Deref Coercion with Mutable References

#[allow(unused_mut)]
fn deref_variations() {
    let x = 1;
    let y = &x;
    println!("y: {}", *y); // &T to &U deref is fine.

    let mut m = 2;
    let mut n = &mut m;
    *n = 3;
    println!("n: {}", *n); // &mut T to &mut U deref is fine.

    let mut a = 4;
    let b = &a;
    println!("b: {}", *b); // &mut T to &U deref is fine.

    /*
        let foo = 5;
        let bar = &mut foo; // cannot borrow `foo` as mutable. Before get to deref.
        *bar = 6;
        println!("bar: {}", *bar); // &T to &mut U deref is not possible due to borrowing rules.
    */
}
