//! Rc<T> - via immutable references, share data between multiple parts of your program for reading only.
//! In the examples below, a linked list is shared/embedded in two other linked lists.

#[allow(unused)]
fn main() {
    box_ownership_issue();
    rc_clone();
    see_ref_count_increases();
}

/// 'b' takes ownership of 'a' so 'c' can't take ownership of 'a'.
#[allow(unused)]
fn box_ownership_issue() {
    {
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        use List::{Cons, Nil};

        let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        // - move occurs because `a` has type `List`, which does not implement the `Copy` trait
        let b = Cons(3, Box::new(a));
        // - value moved here
        // ERROR:
        // let c = Cons(4, Box::new(a));
        // ^ value used here after move
    }
}

/// Use Rc<T> instead so 'b' and 'c' clone references to 'a'.
#[allow(unused)]
fn rc_clone() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};
    use std::rc::Rc; // not in prelude.

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    // xyz.clone() instead of Rc::clone(&xyz) works but is not Rust convention.
    // Using `Rc::clone()` enables visually distinguishing deep copy clones (expensive)
    // versus cheap clones that only increase reference count. For performance problems
    // can thus ignore Rc::clone().
    let d = Cons(7, a.clone());
}

/// Print reference count with `Rc::strong_count(&T)`.
#[allow(unused)]
fn see_ref_count_increases() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};
    use std::rc::Rc; // not in prelude.

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    } // Drop trait for Rc<T> decreases count.
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // xyz.clone() instead of Rc::clone(&xyz) works but is not Rust convention. See above.
    let d = Cons(7, a.clone());
    println!("count after creating d = {}", Rc::strong_count(&a));

    /*
    count after creating a = 1
    count after creating b = 2
    count after creating c = 3
    count after c goes out of scope = 2
    count after creating d = 3
    */
} // `b` and `a` out of scope, count becomes 0, `Rc<List>` is cleaned up completely.
