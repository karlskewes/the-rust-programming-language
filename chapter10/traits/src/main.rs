//! Traits: Defining Shared Behaviour

use aggregator::{notify, send_message, Mail, Message, Pair, Summary, SummaryAuthor, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // println!("1 new tweet: {}", tweet.summarize()); // multiple `summarize` found
    println!("(Summary) 1 new tweet: {}", Summary::summarize(&tweet));
    // 1 new tweet: horse_ebooks: of course, as you probably already know, people

    println!(
        "(SummaryAuthor) 1 new tweet: {}",
        SummaryAuthor::summarize(&tweet)
    );
    // 1 new tweet by author: (Read more from @horse_ebooks...)

    let m = Mail {};

    println!("(Summary) 1 new mail: {}", m.summarize()); // default behaviour.
                                                         // 1 new mail: (Read more...)

    let p = Pair::new('a', 'b');

    println!("p.to_string: {}", p.to_string());
    // println!("p: {}", p); // Pair<char> does not implement std::fmt::Display
    // TBD

    notify(&m);
    // Breaking news! (Read more...)
    notify(&tweet);
    // Breaking news! horse_ebooks: of course, as you probably already know, people

    let msg = Message { x: 1.1, y: 2.2 };
    send_message(msg);
}
