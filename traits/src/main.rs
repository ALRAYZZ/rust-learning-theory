fn main() {
    println!("Hello, world!");
}

// Here we define a trait named `Summary` with a method `summarize`
// And made it public using the `pub` keyword
// Traits don't include implementation details, just the method signatures
// implementation must be done per type that implements the trait
pub trait Summary {
    fn summarize(&self) ->  String;
}

// Implementing a trait on a type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Specific implementation of the Summary trait for NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Specific implementation of the Summary trait for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Default implementations

// Can be useful to have default behavior for some or all methods in a trait
// and if we need to, we can override the default behavior in the specific implementation

// In this case we provide an implementation of the method
pub trait DefaultSummary {
    fn summarize_author(&self) -> String;

    // Also can call other methods in the same trait
    fn summarize(&self) -> String {
        format!(
            "(Read more from {} ...)",
            self.summarize_author()
        )
    }
}


// Traits as parameters

// Use traits to define functions that accept many different types
// Instead of specifying a type, we use the trait name meaning that any type we want to use
// with this function must implement the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Specifying multiple trait bounds with the + syntax
// So here we specify that the item must implement both Summary and DefaultSummary traits
pub fn notify2(item: &(impl Summary + DefaultSummary)) {
    // implementation
}

// Lastly, we can also use the where clause to specify trait bounds when
// there are many of them and the function signature would get too long
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + DefaultSummary,
    U: DefaultSummary + Clone,
{
    42
}

// Returning types that implement traits
// We can also use impl Trait in the return position to return a type that implements a specific
// So  here we specify that the returns_summarizable function will return
// some type that implements the Summary trait without specifying the concrete type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}


// We can also use trait bounds to conditionally implement methods on a type
// Here we implement the Pair struct
pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

// And we implement a new method for Pair<T>
impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// But we only want to implement the cmp_display method if T implements
// both the PartialOrd and Display traits
use std::fmt::Display;

// If type T doesnt implement both traits, this method wont be available
impl<T: PartialOrd + Display> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}