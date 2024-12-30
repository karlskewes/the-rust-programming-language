use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn main() -> Result<(), Box<dyn Error>> {
    // _recoverable_errors();
    // _matching_errors();
    // _closures_with_unwrap();
    // _shortcuts_unwrap();
    // _shortcuts_expect();

    let username = read_username_from_file().expect("File should contain username");
    println!("username: {username}");

    // let username =
    //     read_username_from_file_shortcut().expect("Shortcut: File should contain username");
    //
    // println!("shortcut username: {username}");

    let username =
        read_username_from_file_chaining().expect("Chaining: File should contain username");

    println!("chaining username: {username}");

    let username = read_username_from_file_stdlib().expect("Stdlib: File should contain username");
    println!("stdlib: {username}");

    // let greeting_file = File::open("hello.txt")?;
    // cannot use the `?` operator in a function that returns `()`.

    let last = last_char_of_first_line("\nhi").unwrap_or_default();
    println!("last: {last}");
    // last:

    let last = last_char_of_first_line("hi");
    match last {
        Some(c) => println!("last: {c}"),
        None => println!("last: None"),
    };
    // last: i

    let _greeting_file = File::open("hello2.txt")?;
    Ok(())
}

// Recoverable Errors with Result
fn _recoverable_errors() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    /*
    thread 'main' panicked at src/main.rs:15:23:
    Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    */
}

// Matching on Different Errors
fn _matching_errors() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}

// Alternatives to Using `match` with Result<T, E>
fn _closures_with_unwrap() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

// Shortcuts for Panic on Error: unwrap
fn _shortcuts_unwrap() {
    let _greeting_file = File::open("hello2.txt").unwrap();
    /*
    thread 'main' panicked at src/main.rs:65:50:
    called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    */
}

// Shortcuts for Panic on Error: expect - can define panic message.
// "In production typically chose `expect` over `unwrap`"
fn _shortcuts_expect() {
    let _greeting_file =
        File::open("hello2.txt").expect("hello2.txt should be included in this project");
    /*
    thread 'main' panicked at src/main.rs:76:34:
    hello2.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    */
}

// Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // Example code as follows returns empty String if file created but empty.
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // Check if username defined based on read string length.
    // This error check could also be done in main(), default username, etc.
    match username_file.read_to_string(&mut username) {
        Ok(_) => match username.len() {
            0 => Err(std::io::Error::from(ErrorKind::Other)),
            /*
            thread 'main' panicked at src/main.rs:19:46:
            File should contain username: Kind(Other)
            */
            _ => Ok(username),
        },
        Err(e) => Err(e),
    }
}

// A Shortcut for Propagating Errors: the ? Operator
fn _read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello2.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Chaining method calls with ? operator shortens function.
fn read_username_from_file_chaining() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_stdlib() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
