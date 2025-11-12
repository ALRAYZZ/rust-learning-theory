// Rust supports structs that look similar to tuples, called tuple structs.
// Tuples structs are useful when you want to give the whole tuple a name and make the tuple's fields

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs without any fields, useful for generics
struct AlwaysEqual;

struct NotOwnData {
    active: bool,
    //username: &str,
    //email: &str, // We need to use lifetimes here to ensure the references are valid
    sign_in_count: u64,
}

pub fn run() {
    let black = Color(1, 0, 0);
    let origin = Point(10, 0, 0);

    let origin_x = origin.0;

    let subject = AlwaysEqual;

    println!("{origin_x}");
}
