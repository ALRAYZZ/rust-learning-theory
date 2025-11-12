fn main() {
    let mut s = String::from("hellos worlds");

    let word = frist_word(&s); // word gets the value 5
    s.clear(); // this empties the String, making it equal to ""


    // word still hast the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!
    // but program will compile without any error and word will keep the value 5
    // but this can be PROBLEMATIC in real programs because we cannot trust word anymore
    println!("From {s} string, first word ends at index: {word}");


    // STRING SLICES
    let mut string_slice = String::from("hello world");

    let hello = &string_slice[0..5];
    let hello = &string_slice[..5]; // same as above
    // no numbers at beginning or end, means it's the start of end of the string
    let world = &string_slice[6..];

    let hello_world = &string_slice[..]; // the whole string

    let world = &string_slice[6..11];
    println!("First word slice: {hello}, second word slice: {world}");

    let word_slice = first_word_slice(&string_slice); // immutable reference
    //string_slice.clear(); // mutable reference needed to clear the string
    // compiler will throw an error because we are trying to clear the string while
    // having an immutable reference to it in word_slice
    println!("First word slice from function: {word_slice}");



    // STRING LITERAL SLICES
    let my_string = String::from("hello world"); // string literal is of type &str

    // works on partial or whole string literals
    let word = string_literal_slice(&my_string[0..6]);
    let word = string_literal_slice(&my_string[..]);
    // works on references to 'String' which are equivalent to w hole slices of Strings
    let word = string_literal_slice(&my_string);

    let my_string_literal = "hello world"; // string literal is of type &str

    // works on slices of string literals, whether partial or whole
    let word = string_literal_slice(&my_string_literal[0..6]);
    let word = string_literal_slice(&my_string_literal[..]);

    // because string literals are slices already, this works too
    let word = string_literal_slice(my_string_literal);

}

// this function takes a reference to a String as an argument
// and returns the index of the first space it finds so it can determine
// how long the first word is
fn frist_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter is a method that returns each element in a collection,
    // enumerate wraps the result of iter and returns each element as a tuple
    // where the first element is the index and the second element is a reference to the value
    for (i, &item) in bytes.iter().enumerate() {
        // (b'') is space character as a byte literal, so if we find space return the index of it
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// string slices can also be used with string literals directly
fn string_literal_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
