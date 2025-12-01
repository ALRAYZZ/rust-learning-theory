// by using the use statement on List we don't need to say List::Cons and List::Nil
use crate::List::{Cons, Nil};

fn main() {
    // in this case we are using Box to allocate an integer on the heap
    // b box is a pointer in the stack that points to an integer in the heap
    let b = Box::new(5);
    println!("b = {b}");


    // Using recursive types with Box
    let list = ListNoBox::Cons(1, ListNoBox::Cons(2, ListNoBox::Cons(3, ListNoBox::Nil)));

    let list2 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// Enabling recursive types with Boxes

// This implementation won't compile because the compiler doesn't know how much space
// to allocate for List since it is recursive, it is an infinite size type
enum ListNoBox {
    Cons(i32, ListNoBox),
    Nil,
}

// To fix this we use Box to put the recursive part on the heap
// Box is a smart pointer that allocates data on the heap and stores a pointer to that
// This solves the problem mainly because it moves the data allocation from stack to the heap
// it's not the type itself doing some magic, its just that Box allows us to save the data
// on the heap and only using the stack to store the pointer to that data which is a known size
enum List {
    Cons(i32, Box<List>),
    Nil,
}
