fn run() {
    let mut s = String::new();
    let data = "initial contents";

    // Using to_string method to create a String from a string literal
    let s = data.to_string();
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // Updating a String
    let mut s = String::from("foo");
    s.push_str("bar"); // Appending a string slice to a String

    // push_str does not take ownership of the string slice
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is still usable: {}", s2);

    // Appending a single character to a String
    let mut s = String::from("lo");
    s.push('l'); // push takes a char as an argument

    // Concatenation using the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // Based on add method signature, s1 is moved here and can no longer be used
    // add method takes ownership of the first string and
    // a reference to the second string
    let s3 = s1 + &s2;

    // We can use a format! macro to concatenate multiple strings without taking ownership
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // s1, s2, and s3 are still valid here

    // Indexing into Strings

    // If we try to access parts of a String using indexing,
    // in Rust we will get errors
    let s1 = String::from("hello");
    let h = s1[0]; // Rust strings don't support indexing
    // Main reason is that Rust strings are UTF-8 encoded and
    // some characters may take up more than one byte
    // so accessing by byte index could lead to invalid characters

    // Slicing Strings
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // This works because the first two characters are 4 bytes in UTF-8
    // But carefull when doing so that we don't split a character in half and cause a panic

    // Methods for iterating over Strings
    for c in "3a".chars() {
        println!("{c}"); // This prints "3" and "a" on separate lines
    }

    for b in "3a".bytes() {
        println!("{b}"); // This prints the byte values of '3' and 'a' being 51 and 97 respectively
    }
}