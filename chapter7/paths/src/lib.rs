// Exposing Paths with the `pub` keyword

// front_of_house module can be accessed by eat_at_restaurant because
// they are module 'siblings', defined in same module (root).
mod front_of_house {
    // module child items are private to parent modules by default

    // mod hosting {
    // error[E0603]: module `hosting` is private
    // expose module with `pub` keyword.
    pub mod hosting {
        // fn add_to_waitlist() {}
        // error[E0603]: function `add_to_waitlist` is private
        pub fn add_to_waitlist() {}
    }
}

// can make individual module functions available for other modules too.
// In this case module consumers can call eat_at_restaurant() but not
// access front_of_house directly.
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Starting Relative Paths with `super`

fn _deliver_order() {}

mod back_of_house_super {
    fn _fix_incorrect_order() {
        _cook_order();
        super::_deliver_order(); // access parent module of back_of_house which is `crate`.
                                 // In this example, believe that back_of_house and
                                 // deliver_order will likely have the same relationship and be moved together so
                                 // used 'super'.
    }

    fn _cook_order() {}
}

// Making Structs and Enums Public

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // because Breakfast has a private field we need to provide a public constructor
        // otherwise we couldn't create an instance of Breakfast because we can't set the
        // private field seasonal_fruit.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaraunt_public() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // following line won't compile because seasonal_fruit is not public.
    // meal.seasonal_fruit = String::from("blueberries");
}

// If make an enum public, all of its variants are public.
// It would be annoying to have to annotate each variant with `pub`.
// This contrasts with Structs which have private fields by default and each
// field must be individually annotated with `pub` if require it to be public.
mod back_of_house_enum {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_enum() {
    let _order1 = back_of_house_enum::Appetizer::Soup;
    let _order2 = back_of_house_enum::Appetizer::Salad;
}
