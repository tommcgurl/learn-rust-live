use std::fmt::Display;

// Trait definition without a default implementation.
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// This woul override the default implementation of summarize.
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

// impl Trait Syntax
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// Trait Bound Syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// two args that can have different types
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// two args that have to have the same type
// pub fn notify<T: Summary>(item1: &T, item2: &T) {

// Using the plus syntax to show it must have two traits
// pub fn notify(item: &(impl Summary + Display)) {
    // or
// pub fn notify<T: Summary + Display>(item: &T) {

// Functino without where clause
// fn some_function_without_where_clause<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// Func
// fn some_function_with_where_clause<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// { 4 }

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("tommmcgurl"),
        content: String::from(" We made it to Chapter 10 Rustaceans!!"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {

    let tweet = Tweet {
        username: String::from("tommmcgurl"),
        content: String::from(" Thanks for joining the stream!!"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


    let article = NewsArticle {
        author: String::from("Tom McGurl"),
        content: String::from(" Thanks for joining the stream!! We finally made it to chapter 10!"),
        location: String::from("Twitch.tv"),
        headline: String::from("Thank you"),
    };

    println!("1 new new article: {}", article.summarize());

    // Try using our new notify function
    notify(&article);
    notify(&tweet);
}
