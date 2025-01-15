//! Aggregator is a media aggregator that displays summaries for different media types.
//!
//! Traits are similar to Interfaces in other languages but with some differences... TBD.
//! - coherence
//!
//!   Can only implement traits that are local to your crate, e.g: `Summary` `Trait` on `NewsArticle`.
//!   Can't implement `Summary` `Trait` on `Vec<T>` because `Vec<t>` defined in standard library.
//!   This prevents other peoples code from breaking yours by clashing Trait implementations.

/// Media Summary
pub trait Summary {
    // fn summarize(&self) -> String; // No default behaviour for types without trait
    // implemented.
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

/// News Article
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    /// Generate summary of the news article.
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

/// Tweet
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    /// Generate summary of the tweet.
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Mail {}

impl Summary for Mail {
    // summarize function not defined so uses default Trait behaviour.
    // fn summarize(&self) -> String {
    //     format!("some non default text")
    // }
}

/// Summarize media. Must implement summarize_author() method.
pub trait SummaryAuthor {
    fn summarize_author(&self) -> String;

    // Default implementations can call other methods in the same trait.
    // This means a trait can provide a lot of functionality and only
    // require implementors to specify only a small part of it.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl SummaryAuthor for Tweet {
    // Must implement summarize_author otherwise below error occurs:
    // 1. not all trait items implemented, missing: `fn summarize_author` [E0046]

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

/// Notify new media available.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/// Notify new media available.
/// Above short form is syntax sugar for longer form known as a `trait bound`.
pub fn notify_tb<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/// simple `impl Trait` syntax can do multiple params where each param can be of
/// a different type or the same type. (Whilst still implementing Trait).
pub fn notify_two_params(item1: &impl Summary, item2: &impl Summary) {}

/// only longer `trait bound` syntax can enforce that both params are of the same type `<T>`.
pub fn notify_two_params_same_type<T: Summary>(item1: &T, item2: &T) {}

pub trait Display2 {}

/// Specify multiple trait bounds with the + syntax
pub fn notify_multiple_traits(item: &(impl Summary + Display2)) {}

/// Specify multiple trait bounds with the longer `trait bound` syntax.
pub fn notify_multiple_traits_bounds<T: Summary + Display2>(item: &T) {}

// Clearer Trait Bounds with `where` Clauses
pub trait Debug {}

fn some_function<T: Display2 + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

fn some_function_where<T, U>(t: &T, u: &U) -> i32
where
    T: Display2 + Clone,
    U: Clone + Debug,
{
    0
}

// Returning Types that Implement Traits

/// Returning a type that implements some Trait can be useful for closures and iterators.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// can only use `impl Trait` if you're returning a single type.
// Can't return different types based on a function parameter like below due how compiler works.
// See chapter 17 for alternative implementations with this behaviour:
/*
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
*/

// Using Trait Bounds to Conditionally Implement Methods

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }

        println!("{}", self.to_string());
    }
}

// If T implements std::fmt::Display trait then provide a to_string method, enabling p.to_string().
// NOTE: This does not implement the std::fmt::Display trait.
impl<T: std::fmt::Display> ToString for Pair<T> {
    fn to_string(&self) -> String {
        format!("Pair x: {} y: {}", self.x, self.y)
    }
}

// NOTE: this conflicts with above.
// 1. conflicting implementations of trait `ToString` for type `Pair<_>`
// impl std::fmt::Display for Pair<char> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

pub struct Message {
    pub x: f64,
    pub y: f64,
}

pub fn send_message(m: Message) {
    println!("sent message: {m}");
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
