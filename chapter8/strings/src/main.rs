fn main() {
    // Creating a New String

    let mut _s = String::new(); // new empty string, same method as vectors

    // create a new string with initial contents
    let data = "initial contents";

    let _s = data.to_string();

    // to_string() method works on literal values too.
    let _s = "initial contents".to_string();

    let _s = String::from("initial contents"); // equivalent/alternative to to_string()

    // strings are UTF-8 encoded.
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שלום");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    // Updating a String

    let mut s = String::from("foo");
    s.push_str("bar"); // grow a string like a vector.
    s.push('z'); // push for a single char.

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // takes a string slice, does not take ownership of input.
    println!("s1: {s1} s2: {s2}");

    let mut s = String::from("lo");
    s.push('l');

    // Concatenation with the + Operator or the format! Macro

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // + operator uses add, `fn add(self, s: &str) -> String {}`
    println!("s3: {s3} s2: {s2}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // getting hard to read.
    println!("s: {s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // format! macro cleaner. Python f string style.
    println!("s: {s} - s1: {s1} s2: {s2} s3: {s3}"); // format! macro doesn't take ownership.

    // Indexing into Strings
    let _s1 = String::from("hello");
    // let h = s1[0]; // the type `str` cannot be indexed by {integer}

    // String Internal Representation - Vec<u8> (byte)

    let hello = String::from("Hola");
    println!("len: {}", hello.len()); // 4 (bytes)

    let hello = String::from("Здравствуйте");
    println!("len: {}", hello.len()); // 24 (bytes), each Unicode scalar 2 bytes

    let hello = "Здравствуйте";
    // let answer = &hello[0]; // first byte is 208, second is 151, together make `З`
    // neither 208 nor 151 is a valid character on its own, so what should it return?
    // Avoid returning unexpected values and error instead.
    // Additionally indexing operations expected to be O(1) time, but this isn't possible with
    // strings as need to walk string O(n) to determine how many valid characters.

    // Slicing Strings

    let hello = "Здравствуйте";
    let s = &hello[0..4]; // each character is 2 bytes so at runtime this is ok.

    println!("s: {s}"); // Зд

    // let bad = &hello[0..1];
    /*
        thread 'main' panicked at src/main.rs:90:21:
        byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */

    // Iterating over Strings

    // by chars - unicode returned.
    for c in "Зд".chars() {
        println!("c: {c}"); // c: З... c: д
    }

    // by bytes
    for b in "Зд".bytes() {
        println!("b: {b}"); // b: 208... 151 ... 208... 180
    }
}
