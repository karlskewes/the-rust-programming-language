// `use` keyword

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// bring hosting child module into root scope.
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod customer {
    use crate::front_of_house::hosting;

    pub fn _eat_at_restaurant() {
        // this requires the `use` to be within current scope/module.
        hosting::add_to_waitlist();

        // alternatively, go up to root with `super`
        super::hosting::add_to_waitlist();
    }
}

// unidiomatic use of `use` by referencing modules child function.
// idiomatic way is to keep the parent module namespace like previous example.

mod front_of_house_fn {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house_fn::hosting::add_to_waitlist;

pub fn eat_at_restaurant_fn() {
    add_to_waitlist(); // unidiomatic, what module provides add_to_waitlist?
}

// Structs, Enums and other items idiomatic `use`. Specify full path.
// Convention that has emerged.

use std::collections::HashMap;

fn _main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Exception to idiom is if bringing multiple items with the same name into scope.
// Both fmt and io have a Result type. If don't keep parent namespace then compiler
// won't know what Result type we're referring to.

// use std::io::Result; // error[E0252]: the name `Result` is defined multiple times
use std::fmt;
// use std::fmt::Result;
// |     ---------------- previous import of the type `Result` here
use std::io;
// |     ^^^^^^^^^^^^^^^ `Result` reimported here

fn _function1() -> fmt::Result {
    unimplemented!()
}

fn _function2() -> io::Result<()> {
    unimplemented!()
}

// Providing new names with the `as` keyword - alias.

use std::fmt::Result;
use std::io::Result as IoResult;

fn _function1_as() -> Result {
    unimplemented!()
}

fn _function2_as() -> IoResult<()> {
    unimplemented!()
}

// Re-exporting names with `pub use`
// With `pub use` can write code with one structure organized for module developers
// and expose a different structure organized for module consumers.
// THOUGHTS: could get messy, could enable development whilst preserving original module API... ?

mod front_of_house_reexport {
    pub mod hosting_reexport {
        pub fn add_to_waitlist() {}
    }
}

// without `pub use` external callers would need to call:
//    restaurant::front_of_house::hosting::add_to_waitlist()
// and mod front_of_house_reexport would need to be public.
pub use crate::front_of_house_reexport::hosting_reexport;

pub fn eat_at_restaurant_reexport() {
    hosting_reexport::add_to_waitlist();
}
