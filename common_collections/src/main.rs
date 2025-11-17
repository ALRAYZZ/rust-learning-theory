mod strings;

fn main() {
    // VECTORS
    // Creating a new empty vector of integers
    let v: Vec<i32> = Vec::new();

    // Inferring the type of the vector using vec! macro
    let v2 = vec![1, 2, 3, 4];

    // Updating vector
   let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8); // Vector needs to be mutable to push new elements

   v2.push(9); // This will cause a compile-time error because v2 is not mutable

    // Reading elements of Vectors

    let mut v3 = vec![1, 2, 3, 4];
    // Using indexing to access the third element
    let third: &i32 = &v3[2]; // This creates an immutable reference to the third element
    let third2: i32 = v3[2];
    // Even if the vector is defined as mutable,
    // if we have an immutable reference to one of its elements,
    // we cannot modify the vector until that reference goes out of scope.
    v3.push(6);

    println!("The third element is {}", third);

    // Using get method to access the second element
    let second: Option<&i32> = v.get(1);
    match second {
        Some(second) => println!("The second element is {}", second),
        None => println!("There is no second element."),
    }

    // Accessing out of bounds element

    // This will cause a runtime panic
    let does_not_exist = &v3[100];
    // This will return None so no panic occurs and programs continues to run
    let does_not_exist2 = v3.get(100);

    // Iterating over the values in a vector
    let v4 = vec![100, 32, 57];
    // Get immutable references to each element
    for i in &v4 {
        println!("{i}");
    }


    let mut v5 = vec![100, 32, 57];
    // Get mutable references to each element
    for i in &mut v5 {
        // We need to dereference i to change its value from the mutable reference
        *i += 50;
    }

    // Vectors only allow to store same types but we can combine
    // enums and vectors to store multiple types

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    // Creating a vector of SpreadsheetCell enum so we can store multiple types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Dropping elements from a vector
    {
        let mut v6 = vec![1, 2, 3, 4, 5];
    } // v6 goes out of scope and is dropped here


}
