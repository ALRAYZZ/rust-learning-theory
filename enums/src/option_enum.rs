fn run() {
    let some_number = Some(5);
    let some_char = Some('e');

    // This is the NULL equivalent in Rust
    // We need to tell Rust what type of data we plan to store in the Option enum
    // Since it cant infer the type from None alone
    let absent_number: Option<i32> = None; // Using the Option enum from standard library
}