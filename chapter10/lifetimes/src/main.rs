//! Validating References with Lifetimes

fn main() {
    // Lifetime Annotation Syntax
    /*
        &i32 // reference
        &'a i32 // reference with an explicit lifetime
        &'a mut i32 // mutable reference with an explicit lifetime
    */

    lifetime_check();
    generic_lifetimes_in_functions();
    lifetime_annotations_in_structs();
    lifetime_three_rules();
    lifetime_annotations_in_methods();
    the_static_lifetime();
    generic_type_params_trait_bounds_lifetimes();
}

/// Prevent dangling references with lifetimes.
fn lifetime_check() {
    /*
        let r;

        {
            let x = 5; // binding `x` declared here
            r = &x; // `x` does not live long enough
        } // `x` dropped here while still borrowed

        println!("r: {r}"); // borrow later used here
    */

    let x = 5; // ----------+-- 'b
               //           |
    let r = &x; // --+-- 'a  |
                //   |       |
    println!("r: {r}"); //   |       |
                        // --+       |
} // ----------+

// Generic Lifetimes in Functions
fn generic_lifetimes_in_functions() {
    // Generic Lifetimes in Functions
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {result}");
    }

    // passing in references with different concrete lifetimes.
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // the following doesn't compile because result is used outside
    // of the lifetime constraint required by longest()
    /*
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
            // ^^^^^^^ borrowed value does not live long enough
        }
        println!("The longest string is {result}");
    */
}
/// longest takes two string slices and returns the longest of the two. The return result is valid
/// as long as both parameters are valid. This is the lifetime relationship between them and this
/// constraint is captured by the lifetime `'a`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // fn longest(x: &str, y: &str) -> &str {
    // expected named lifetime parameter
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// y does not impact lifetime of result so lifetime parameter 'a does not need to be applied to
// the y parameter.
fn longest1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// following doesn't compile because result is declared within the function and its lifetime/scope
// ends after the function completes, thus a dangling reference which Rust won't let us create.
// The returned value needs to either be a static knowable value (length of x, int, etc) or have a
// lifetime tied to an input parameter.
/*
fn longest_broken<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
*/

// Structs can hold owned types, they can also hold references but we need to add lifetime
// annotations on every reference in the struct definition.
fn lifetime_annotations_in_structs() {
    struct ImportantExcerpt<'a> {
        part: &'a str, // instance of ImportantExcerpt cannot outlive lifetime of part.
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// Lifetime Elision
fn first_word_pre_rust1<'a>(s: &'a str) -> &'a str {
    // <snip>
    s
}

// Pre Rust 1.0 the following wouldn't have compiled and would have required lifetime annotations
// like above function. However, some lifetimes are deterministic and so to make developing in Rust
// easier some lifetime annotations can be omitted and the Rust compiler will correctly infer.
// These supported patterns are called 'lifetime elision rules'.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/// Lifetimes will attempt to be inferred by the compiler based on three rules.
/// 1. each parameter that is a reference is assigned its own lifetime
/// 2. if exactly one input lifetime parameter, that is assigned to all output lifetime
///    parameters
/// 3. if multiple input but one is &self or &mut self, then this is a method and all output
///    lifetime parameters are assigned to self's lifetime.
/// 4. <unspecified> but if not satisfied, then fail compilation and suggest fix.
fn lifetime_three_rules() {
    fn _first_word(s: &str) -> &str {
        s
    }

    fn _first_word_after_one<'a>(s: &'a str) -> &str {
        s
    }

    fn _first_word_after_one_and_two<'a>(s: &'a str) -> &'a str {
        s
    }

    // Despite compilers lifetime elision rules the output lifetime parameter could not be
    // inferred and compilation failed: `consider introducing a named lifetime`
    /*
    fn longest(x: &str, y: &str) -> &str {}
    fn longest_after_one<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
    fn longest_after_one_and_two<'a, 'b>(x: &'a str, y: &'b str) -> &str { /* no change */
    }
    fn longest_after_one_two_three<'a, 'b>(x: &'a str, y: &'b str) -> &str { /* no change */
    }
    */
}

/// lifetime names for struct fields always need to be declared after the `impl` keyword because
/// those lifetimes are part of the struct's type.
fn lifetime_annotations_in_methods() {
    struct ImportantExcerpt<'a> {
        part: &'a str, // instance of ImportantExcerpt cannot outlive lifetime of part.
    }

    impl<'a> ImportantExcerpt<'a> {
        // level does not need a lifetime declaration because of the first rule of
        // 'lifetime elision rules'.
        fn level(&self) -> i32 {
            3
        }

        // first rule, each input param gets own lifetime.
        // third rule, one input param is &self so return type gets &self lifetime (from struct).
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {announcement}");
            self.part
        }
    }
}

/// static lifetime annotated values are stored in the program's binary, which is always available.
/// Thus the `'static` lifetime denotes that the reference can live for the entire duration of the
/// program.
fn the_static_lifetime() -> &'static str {
    // All string literals have a 'static lifetime, per below:
    let s: &'static str = "I have a static lifetime.";
    // Compiler infers for us so below is equivalent and easier to type.
    // let s: &str = "I have a static lifetime.";

    s
}

use std::fmt::Display;
fn generic_type_params_trait_bounds_lifetimes() {
    // output lifetime is as long as shortest of input params x & y
    // lifetimes are a type of generic so go in <> alongside type param T.
    fn _longest_with_an_announcement<'a, T>(
        x: &'a str, 
        y: &'a str, 
        ann: T,
    ) -> &'a str
    where
        T: Display, // announcement type must implement Display trait.
    {
        println!("Announcement: {ann}");
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
