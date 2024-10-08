fn main() {
    // Finding first word in a string (slice)
    let mut s = String::from("hello world");

    let word = first_word_end_index(&s); // word will get the value 5 (index)
    println!("word: {word}");

    s.clear(); // empties string and makes word value invalid!

    // String Slices - Assumes ASCII only!!

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("h: {hello}, w: {world}");

    // hello[0] = 'z'; // the type str cannot be indexed by {integer}

    // start from 0, drop slice 0 beginning number
    let s = String::from("hello");

    let slice = &s[0..2];
    println!("slice: {slice}");
    let slice = &s[..2]; // equivalent
    println!("slice: {slice}");

    // including last byte, drop trailing number

    let s = String::from("hello");

    let len = s.len();
    let slice = &s[3..len];
    println!("slice: {slice}");
    let slice = &s[3..s.len()];
    println!("slice: {slice}");
    let slice = &s[3..]; // all above equivalent
    println!("slice: {slice}");

    // take slice of whole string without beginning or trailing

    let s = String::from("hello");

    let slice = &s[0..s.len()];
    println!("slice: {slice}");
    let slice = &s[..];
    println!("slice: {slice}");

    // String slice indices must occur at valid UTF-8 character boundaries.
    // If attempt index in middle of multi-byte character, program will exit
    // with an error.

    let s = String::from("hello world, how are you doing?");
    let word = first_word(&s);
    println!("word: {word}");

    let phrase = String::from(&s[word.len() + 1..]); // note +100 index out of bounds
    println!("phrase: {phrase}");
    let second_word = first_word(&phrase);
    println!("second_word: {second_word}");

    // first_word API takes String and compiler ensures references stay valid.
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error! can't mutate s due to below immutable borrow usage.

    println!("the first word is: {word}");

    // String Literals as Slices

    let s = "Hello, world!"; // s is &str, reference to slice in the binary (location in binary), i.e: baked into
    // binary
    // This is also why string literals are immutable. &str is an immutable reference.

    // String Slices as Parameters

let my_string = String::from("hello world");

    // `first_word` works on slicess of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent to whole slices
    // of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world"; // &str
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    let word = first_word(&my_string_literal);

    println!("word: {word}");

    let word = first_word(my_string_literal);


    // Other Slices

    let a = [1,2,3,4,5];

    let slice = &a[1..3];
    println!("slice: {slice:?}"); // slice: [2, 3]

    assert_eq!(slice, &[2, 3]); // no panic
}

// fn first_word(s: &String) -> &str { // original signature.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s;
}

// fn first_word(s: &String) -> ? // how to return part of string?
fn first_word_end_index(s: &String) -> usize {
    // return index of end of word
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
