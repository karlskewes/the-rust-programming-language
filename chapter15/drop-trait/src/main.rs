fn main() {
    println!("--- START custom_drop() ---");
    custom_drop();
    println!("--- END custom_drop() ---");

    println!("--- START calling_drop() ---");
    calling_drop();
    println!("--- END calling_drop() ---");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn custom_drop() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");

    /* NOTE: LIFO
    CustomSmartPointers created
    Dropping CustomSmartPointer with data `other stuff`!
    Dropping CustomSmartPointer with data `my stuff`!
    */
}

// Force a value to be dropped before the end of its scope by calling `drop(my_var)`.
// Useful when might want to release a lock early so later code can acquire.
fn calling_drop() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    // c.drop() // explicit destructor calls not allowed.
    drop(c); // std::mem::drop, not trait Drop!
    // drop(c); // use of moved value. Can't call twice.
    println!("CustomSmartPointer dropped before the end of function");
    /*
    CustomSmartPointer created
    Dropping CustomSmartPointer with data `some data`!
    CustomSmartPointer dropped before the end of function
    */
}
