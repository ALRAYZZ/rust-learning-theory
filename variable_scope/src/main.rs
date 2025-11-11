fn main() {
    let _s = "Hello!"; // This is a string slice allocated on the stack

    //&str: Immutable view/reference, fixed size, points to string data somewhere else
    //String: Owned, mutable, growable, heap-allocated buffer

    // We can modify String but not &str
    // String type can manage an amount of text that is unknown at compile time
    // We can create a String from a string literal (&str) using the to_string method or String::from function
    let _literal = String::from("hello"); // This is a String  allocated on the heap
    let _string = "Hello".to_string();

    {                               // r is not valid here, not declared
        let _r = "hello";      // r is valid from this point forward

        // do stuff with r
    }                               // r is no longer valid here


    // String is mutable, we can change its content
    let mut x = String::from("hello");
    x.push_str(", world!");
    println!("{x}");


    // string literals are known at compile time and stored in the binary's read-only memory
    // so they are fast and efficient.

    // With String type we can support a mutable, growable piece of text,
    //but it comes with some overhead because it needs to allocate memory on the heap

    // When a String goes out of scope, Rust automatically calls the drop function to free the memory
    {
        let _s1 = String::from("hello");
    }

    // For simple types like integers, Rust uses the Copy trait both are stored on the stack
    // so copying them is fast and cheap
    // Here are some of the types that implement Copy:
    //
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
    let x = 5;
    let _y = x; // Copy trait, x is still valid after this line


    let s2 = String::from("hello");
    // when s2 moved to s3, s2 gets free and is no longer valid
    let s3 = s2; // Move trait, s2 is no longer valid after.
    println!("{s3}");

    let mut drop = String::from("hello");
    drop = String::from("ahoy!"); // previous value "hello" is dropped here
    println!("{drop}");

    // To make a deep copy of a String, we can use the clone method
    // this is expensive because it allocates new memory and copies all the data of the heap
    let string_clone = String::from("hello world");
    let string_clone2 = string_clone.clone(); // deep copy, both are valid
    println!("string_clone = {string_clone}, string_clone2 = {string_clone2}");


    // FUNCTION OWNERSHIP

    let function_s = String::from("hello");

    takes_ownership(function_s); // function_s value moves into the function
                                // function_s is no longer valid here

    println!("{function_s}"); // this would cause a compile-time error, function_s is not valid here

    let x = 5;              // x comes into scope
    makes_copy(x);         // x would move into the function,
                            // but i32 is Copy, so it's okay to still
                            // use x afterward
    println!("{x}");


    let _o1 = gives_ownership();  // gives_ownership moves its return value into o1

    let o2 = String::from("hello");

    let _s3 = takes_and_gives_back(o2); // o2 is moved into takes_and_gives_back,
                                        // which also moves its return value into s3
} // Here, o1 and s3 go out of scope and are freed. o2 goes out of scope but was moved, so nothing happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and moves out to the caller
}

// This function takes a String and returns it
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string  // a_string is returned and moves out to the caller
}


