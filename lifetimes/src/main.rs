// Must specify lifetimes when a function returns a reference that comes
// from more than one input reference.

// Mainly because it depends on runtime conditions so Rust can't determine
// which input lifetime applies

// IT'S WHEN WE RETURN A REFERENCE, NOT WHEN WE JUST USE THEM INSIDE THE FUNCTION

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest_with_lifetime(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn generic_lifetime() {
    let r;

    {
        let x = 5;
        r = &x; // Error: `x` does not live long enough
    } // `x` goes out of scope here, so r becomes a dangling reference

    // Attempting to use `r` here would be a dangling reference
    println!("r: {r}");


    // Generic Lifetimes in Functions
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}


// This implementation will not compile because the returned reference
// may outlive the references passed to the function.
// the borrow checker cannot guarantee that the returned reference
// will be valid for as long as needed. He cannot check as we did on the main function
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }

}

// Lifetime Annotations Syntax
// &i32 -> a reference
// &'a i32 -> a reference with an explicit lifetime 'a
// &'a mut i32 -> a mutable reference with an explicit lifetime 'a


// Lifetime Annotations in Function Signatures

// In this case, both input references and the output reference
// must live at least as long as the lifetime 'a.
// This means that the returned reference will be valid
// as long as both input references are valid.
// For this function to be safe, the returned reference
// must only be valid if both input references are valid.

// The general rule here is that
// The returned reference must live no longer than the shortest of x or y.
fn longest_with_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime annotations in structs

// This annotation means that an instance of ImportantExcerpt
// cannot outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn struct_test() {
    let novel = String::from(
        "Call me Ishmael. Some years ago..."
    );

    let first_sentence = novel
        .split(".")
        .next()
        .expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// Here we have only one input lifetime parameter and
// therefore the return type gets the same lifetime.
impl <'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// Two input lifetimes and one output lifetime, using elision rules, gives both
// inputs their own lifetimes, then because one parameter is self,
// the return type gets the lifetime of self.
impl <'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

//  Static Lifetimes
// A reference with the 'static lifetime can live for the entire duration of the program.
// All string literals have the 'static lifetime
let s: &'static str = "I have a static lifetime.";