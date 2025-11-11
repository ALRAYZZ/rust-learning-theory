// A reference is like a pointer in that it’s an address we can follow to access the data stored at
// that address; that data is owned by some other variable. Unlike a pointer, a reference is
// guaranteed to point to a valid value of a particular type for the life of that reference.


fn main() {
    let s1 = String::from("hello");

    // Original variable must be mutable to create a mutable reference
    let mut s = String::from("hello");

    let len = calculate_length(&s1); // s1 is not moved, we pass a reference to it,
                                            // we can keep using s1 after this call

    println!("The length of '{s1}' is {len}.");

    //change_default_reference(&s1);
    _change_mutable_reference(&mut s);
}

// usize is an unsigned integer type that’s used for indexing, sizes,
// and memory-related values in Rust.
// It’s essentially the type used by the system to represent the size of objects in memory,
// or an index within a collection.
fn calculate_length(s: &String) -> usize { // Takes a reference to a String and returns its length
    s.len()
} // Here s goes out of scope. But because it does not have ownership of what
  // it refers to, the String is not dropped.

// This function will cause a compile-time error because it tries to
// modify a borrowed value through an immutable reference.
fn change_default_reference(some_string: &String) {
    some_string.push_str(", world");
}

fn _change_mutable_reference(some_string: &mut String) {
    some_string.push_str(", world");
}