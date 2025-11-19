fn main() {
    // Removing duplication by extracting a function

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");
}

// Extracted function to find the largest number in a list
// We use references to avoid taking ownership of the list
// Since we don't wanna modify the list, we use an immutable reference
fn find_largest(list: &[i32]) -> &i32  {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generic Data Types
// In Functions definitions

// We can also avoid duplicated code with different data types by using generics

// Adding the trait bound PartialOrd to T tells the compiler
// that we will only use types that implement the PartialOrd trait,
// which is necessary to use the > operator to compare two values of type T.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
