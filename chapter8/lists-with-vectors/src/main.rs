enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Creating a Vector, stored in the heap. TBD, always?

    let _v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3]; // initialize with vec! macro. Type inferred.

    println!("v: {:?}", v);
    // v: [1, 2, 3]

    // Updating a Vector

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading Elements of Vectors

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    // thread 'main' panicked at src/main.rs:34:28:
    // index out of bounds: the len is 5 but the index is 100

    let does_not_exist = v.get(100); // Some or None

    match does_not_exist {
        Some(number) => println!("The 100th element is {number}"),
        None => println!("There is no 100th element"),
    }
    // There is no 100th element

    #[allow(unused_mut)] // silence unused_mut linter warning because v.push(6) commented.
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // immutable borrow to vector

    // v.push(6); // cannot borrow `v` as mutable because immutable borrow above.
    // vectors put values contiguous in memory, so v.push() will result in a
    // new memory allocation and copying all the elements across, invalidating
    // `first` memory reference (now deallocated memory).

    println!("The first element is: {first}");

    // Iterating Over the Values in a Vector

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{i}");
    }

    // Using an Enum to Store Multiple Types

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Dropping a Vector drops its elements

    {
        let v2 = vec![1, 2, 3, 4];

        for i in &v2 {
            println!("{i}");
        }
    } // v goes out of scope here and is freed.

    // let val = &v2[0]; // cannot find value `v2` in this scope
}
