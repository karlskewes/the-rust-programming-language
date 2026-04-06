use self::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
#[allow(unused)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn create_ref_cycles() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    /*
    a initial rc count = 1
    a next item = Some(RefCell { value: Nil })
    a rc count after b creation = 2
    b initial rc count = 1
    b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
    b rc count after changing a = 2
    a rc count after changing a = 2
    */
    // After main(), b will reduce to 1, a will reduce to 1, but neither will reduce to 0.
    // This is a memory leak.

    // uncomment to see that we have a cycle; it will overflow stack.
    // println!("a next item = {:?}", a.tail());
    /*
    <snip>
    Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell {

    thread 'main' (86088) has overflowed its stack
    fatal runtime error: stack overflow, aborting
    Aborted                    (core dumped) cargo run
    */
}
