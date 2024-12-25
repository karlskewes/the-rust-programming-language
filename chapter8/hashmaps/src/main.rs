use std::collections::HashMap;

fn main() {
    // Creating a new hash map, data stored on heap like a vector.
    // All keys must be the same type, all values also the same type (different to keys ok)
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    // Accessing Values

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("team_name: {team_name} score: {score}");

    // iterate over, arbitrary order (TBD: always arbitrary? on purpose like Go?)
    // possible to iterate ordered without map.keys() -> sort() -> for {}?
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Ownership
    // types that implement Copy trait like i32 are copied into map, owned values like String
    // are moved into the hash map and ownership changes.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{field_name}: {field_value}"); // borrow of moved value

    // Overwrite a key

    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Blue".to_string(), 25);
    println!("{scores:?}"); // {"Blue": 25}

    // Add only if key not present

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
    // {"Blue": 10, "Yellow": 50}

    *scores.entry(String::from("Blue")).or_insert(999) *= 2; // default '999' not used if
                                                             // exists.
    *scores.entry(String::from("Red")).or_insert(999) *= 2; // default '999' not used if
    println!("{scores:?}");
    // {"Blue": 20, "Yellow": 50, , "Red": 1998} // note Red default 999 * 2

    // Updating a value based on an old value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
    // {"wonderful": 1, "hello": 1, "world": 2}

    // Summary

    let mut list = vec![5, 2, 2, 3, 5, 6, 7, 9, 1, 1, 2, 1];
    list.sort();
    println!("list: {list:?}");

    let median = list.get(list.len() / 2);
    match median {
        Some(number) => println!("median: {number}"),
        None => println!("median: empty list"),
    }

    let mut mode = (0, 0); // mode, count, defaulted. Anonymous struct instead?
    let mut counts = HashMap::new();
    for item in list {
        let count = counts.entry(item).or_insert(0);
        *count += 1;

        if *count > mode.1 {
            // 1 and 2 have same count, interestingly over ~10 iterations, 1 is always chosen.
            mode = (item, *count);
        }
    }

    println!("counts: {counts:?}");
    println!("mode: {} count: {}", mode.0, mode.1);
}
