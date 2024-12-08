#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// define implementation.
impl Rectangle {
    // first method argument always self, like Python. Contrast with Go.
    // `&self` short for `self: &Self`.  `Self` is an alias for type `impl` is for.
    fn area(&self) -> u32 {
        // immutable borrow of instance. Can also do mutable borrow `&mut self`.
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // no self param, common for constructors that return new instance of struct.
    // Generally called `new` by convention.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// separate impl <struct> blocks are possible.
// It doesn't make sense here, but later apparently...
impl Rectangle {
    fn example_in_another_impl_block() -> bool {
        true
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    // The area of the rectangle is 1500 square pixels.

    if rect1.width() {
        // note: automatic referencing, no need to (&rect1).width() or something.
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Call namespaced function. Like String::from()
    // let sq = Rectangle::square(3);
    println!("Square is: {} square pixels", Rectangle::square(20).area());

    println!("Example: {}", Rectangle::example_in_another_impl_block());

}
