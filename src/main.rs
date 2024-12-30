#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U, // x & y can be different types with generics
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

pub trait Summary {
    fn summarize(&self) -> String; // traits can also have default implementations https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn notify(item: &impl Summary) { // takes any type that implements summary
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!["a", "b", "c"];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let point = Point { x: 5, y: 10.0 };
    println!("The largest point is at {}, {}", point.x(), point.y());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
}