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


// In Struct definitions

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn make_point() {
    // In this case we use same struct for integers as well as floats
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // This will throw an error because x and y are of different types
    let mix = Point { x: 5, y: 4.0 };
    let mix2 = Point2 { x: 5, y: 4.0 }; // Using 2 generic types

    // Using the method defined below fn x()
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x);
}

// In Enum definitions

// The Option enum is defined with a generic type T
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// In Method definitions

// Reusing Point struct
// Here we use the generic type T in the method definition
// We need to do impl<T> to specify that we are implementing methods for Point<T>
// and is a generic type not a concrete type
impl<T> Point<T> {
    fn x (&self) -> &T {
        &self.x
    }
}
// We can also implement methods for specific types that only the Point struct with that type can use
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}