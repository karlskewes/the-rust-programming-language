use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
#[allow(unused)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use crate::cons::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;
    #[test]
    fn test_mutable_borrow() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        assert_eq!(
            format!("a after = {a:?}"),
            "a after = Cons(RefCell { value: 15 }, Nil)"
        );
        assert_eq!(
            format!("b after = {b:?}"),
            "b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))"
        );
        assert_eq!(
            format!("c after = {c:?}"),
            "c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))"
        );
    }
}
