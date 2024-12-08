struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs, own type without field names
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like structs without any fields
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        // entire instance of struct must be mutable, can't restrict to subset
        // of fields.
        active: true,
        username: String::from("someusername123"), // String instead of &str means struct owns its
        // data. Also don't need to manage lifetimes, see chapter10. Also move in user3 below.
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // println!("{}", user1); // User doesn't implement std::fmt::Display

    user1.email = String::from("anotheremail@example.com");

    // println!("{}", user1);

    // create a new user instance using most values from another

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("different_value@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // or with "struct update" syntax `..`

    let user3 = User {
        email: String::from("different_value@example.com"),
        ..user2 // MOVE. user1 already "moved" with user2 above, if delete the user2 declaration
        // can use user1. String doesn't implement Copy trait.
    };


    // Tuple struct
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    // these are not interchangeable as different types, despite same values. Casting?!?


    // Unit-like struct useful when need to implement a trait on some type but don't have any data
    // want to store on the type itself.
    let subject = AlwaysEqual;


}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // parameter name and struct field name same so don't need to repeat.
        email,
        sign_in_count: 1,
    }
}

// long initialization method specifying all fields
fn build_user_long(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
