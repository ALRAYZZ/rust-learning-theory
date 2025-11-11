// A reference is like a pointer in that it’s an address we can follow to access the data stored at
// that address; that data is owned by some other variable. Unlike a pointer, a reference is
// guaranteed to point to a valid value of a particular type for the life of that reference.


fn main() {
    let s1 = String::from("hello");

    // Original variable must be mutable to create a mutable reference
    let mut s = String::from("hello");

    let len = calculate_length(&s1); // s1 is not moved, we pass a reference to it,

    println!("The length of '{s1}' is {len}.");

    // This will cause a compile-time error because we are trying to
    // modify a borrowed value through an immutable reference.
    //change_default_reference(&s1);

    // This works because we are passing a mutable reference
    change_mutable_reference(&mut s);

    // Immutable references can be multiple
    let or = String::from("hello");
    let r1 = &or; // no problem
    let r2 = &or; // no problem

    // Mutable references are only allowed if there are no other references
    let mut original = String::from("hello");
    let r1 = &mut original;
    //let r2 = &mute original; // ERROR: cannot borrow `original` as mutable more than once at a time


    // We can use brackess to create a new scope, which allows us to have
    // a mutable reference while the immutable references are no longer used.

    let mut option = String::from("hello");
    {
        let op1 = &mut option; // no problem
    } // op1 goes out of scope here, so we can make a new mutable reference
    let op2 = &mut option; // no problem

    // Combining mutable and immutable references
    let mut combine = String::from("hello");

    let c1 = &combine; // no problem
    let c2 = &combine; // no problem
    //let c3 = &mut combine; // ERROR: cannot borrow `combine` as mutable because it is also borrowed as immutable


    let mut print = String::from("hello");

    let p1 = &print; // no problem
    let p2 = &print; // no problem
    println!("{p1} and {p2}"); // Compiler can drop the immutable references here if last used
    // After the last use of the immutable references, we can create a mutable reference
    let p3 = &mut print; // no problem

    // This line will cause an error if uncommented, because p1 and p2 are still in use.
    // compiler assumed that previous print was the last use of p1 and p2
    // so it allowed the creation o f a new mutable reference.
    // println!("{p1}, {p2}, and {p3}");
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
    // ERROR: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    // some_string.push_str(", world");
}

// This function takes a mutable reference to a String and modifies it.
fn change_mutable_reference(some_string: &mut String) {
    some_string.push_str(", world");
}

// This function will not compile because it returns a reference to a value
// that will go out of scope when the function ends.
fn dangle() -> &String { // dangle returns a reference to a string
    let s = String::from("hello"); // s is created here

    &s // we return a reference to s
} // Here, s goes out of scope and is dropped. Its memory is freed.
    // Therefore, the reference we returned points to invalid memory.


fn no_dangle() -> String {
    let s = String::from("hello");

    s // s is returned and moves out to the calling function
} // Solution is to return the String directly. The ownership of the String
  // is transferred to the calling function, so it remains valid.