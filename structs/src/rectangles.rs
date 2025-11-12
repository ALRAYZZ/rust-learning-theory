#[derive(Debug)] // This annotation will allow us to print the struct using {:?}
struct Rectangle {
    width: u32,
    height: u32,
}


pub fn run() {
    let scale = 2;
    let width1 = 30;
    let height1 = 50;

    // Using a tuple to represent rectangle dimensions
    // problem with structs is that make our code less readable, we need
    // to remember what each element in the tuple represents and its order
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", simple_area(rect1));

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));


    // USING STRUCTS
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // dbg! macro prints the value and returns it
        height: 50,
    };
    // By passing the rect2 as reference we can continue to use rect2 in main if we needed to
    println!("The area of the rectangle is {} square pixels.", struct_area(&rect2));

    // DEBUG PRINTING
    println!("rect2 is {:?}", rect2); // {:?} tells println! to use the debug format
    println!("rect2 is {:#?}", rect2); // Pretty print with new lines and indentation

    // Need to pass reference since dbg can take ownership of the value passed to it
    dbg!(&rect2); // dbg! macro prints to stderr with file and line number info

}

fn area (width: i32, height: i32) -> i32 {
    width * height
}

fn simple_area (dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

fn struct_area (rect: &Rectangle) -> u32 {
    rect.height * rect.width
}