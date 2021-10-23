use std::fmt::{Debug, Display};

pub fn run() {
    // let integer: Point<i32> = Point {x: 5, y: 10};
    // let float: Point<f32> = Point {x: 1.0, y:4.0};
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "lorem ipsum"
        ),
        reply: false,
        retweet: false,
    };

    // let s: &'static str = "I have a static lifetime.";

    let a1 = returns_summarizable();
    println!("{}", a1.summarize());

    let num_list = vec![34,50,25,100,65];
    // println!("Largest number in num list is {}", largest(&num_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    // println!("The largest char is {}", largest(&char_list));

    // notify(&tweet);

    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);

    // let novel = String::from("Call me Ishmal. Some yeras ago..");
    // let first_sentence = novel.split('.').next().expect("Could not find sentence");
    // let i = ImportantExcerpt {
    //     part: first_sentence
    // };
}

// fn notify(item: &impl Summary) {
//     println!("breaking news! {}", item.summarize());
// }
// fn notify(item: &(impl Summary + Display))
// fn notify<T: Summary + Display>(item: &T)


// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
            U: Clone + Debug
{
    1
}

struct Hello<T, U> {
    a: T,
    b: U
}

impl<T, U> Hello<T, U> {
    fn mixup<V, W>(self, other: Hello<V, W>) -> Hello<T, W> {
        Hello {
            a: self.a,
            b: other.b,
        }
    }
}


// struct Config {}

// struct App<'a> {
//     config: &'a Config
// }
//
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
//
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,

    }
}

trait Summary {
    fn summarize(&self) -> String;
}

// fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
// this is verbose
// fn notify<T: Summary>(item: &T) {
//     item.summarize();
// }

// Specifying Multiple Trait Bounds with the + Syntax
// fn notify(item: &(impl Summary + Display)) {
// fn notify<T: Summary + Display>(item: &T) {

// Clearer trait bounds with "where" Clauses
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// equals to
// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {


struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// generic struct
struct Point<T> {
    x: T,
    y: T
}

// generic methods
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Point with type f32 only can use this method
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
