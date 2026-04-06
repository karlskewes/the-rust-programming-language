//! Prevent reference cycles using Weak<T>.
//! Call Rc::downgrade to get a weak reference, then Rc::upgrade to get an Option<Rc<T>>.

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub fn part_1_child_cant_access_parent() {
    #[derive(Debug)]
    #[allow(unused)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    println!("leaf = {:?}", leaf);

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    println!("branch = {:?}", branch);
}

pub fn part_2_child_can_access_parent() {
    #[derive(Debug)]
    #[allow(unused)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
        parent: RefCell<Weak<Node>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

pub fn part_3_visualizing_changes() {
    #[derive(Debug)]
    #[allow(unused)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
        parent: RefCell<Weak<Node>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
