// created with `cargo new restaurant --lib`

mod front_of_house {
    mod hosting {
        // within a module can hold:
        // structs, enums, constants, traits, functions, modules, etc

        fn _add_to_waitlist() {}

        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}

/* module tree - akin to a filesystem tree
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
*/
