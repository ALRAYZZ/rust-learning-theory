use std::ops::Deref;

fn run() {

}

// Following the pointer to the value
fn reference_to_i32() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // y it's a reference, we need to use the dereference operator to get the value that it points to
    assert_eq!(5, *y);
}

// Using Box<T> like a reference
fn box_to_i32() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Defining our own smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// This is the common way to implement deref for our own smart pointer that we define
// with this we can use the dereference operator on MyBox instances
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Using MyBox like a reference
fn mybox_to_i32() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // we can use *y because we implemented Deref for MyBox
    assert_eq!(5, *y);
}


// Deref coercion
// Rust compiler will analyze the types and automatically call deref method if they implemented
// it will run as many times as needed in compile time, so no runtime cost is added
fn deref_coercion(name: &str) {
    println!("Hello, {name}!");
}

fn hello() {
    let m = MyBox::new(String::from(("Rust")));
    // m is a reference to MyBox<String>
    // but deref coercion will convert it to &String and then to &str
    deref_coercion(&m);
}