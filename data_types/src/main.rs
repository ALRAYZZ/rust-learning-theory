fn main() {

    // SCALAR TYPES
    let guess :u32 = "42".parse().expect("Not number!");

    let x = 0.2;

    let y :f32 = 0.3;
    // Cannot subtract f64 and f32 directly
    let  difference = y - x;

    let new_difference = 95.6 - 33.2;

    let is_running = true;
    let is_big:bool = false;

    let c = 'z';
    let string = "Hello";
    let z:char = 'Z';

    // COMPOUND TYPES

    // Tuple type
    // Once declared, cannot change type or length
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructuring, breaking a tuple into its parts
    let (x, y, z) = tup;
    println!("The value of x is : {x}");

    let test: (i32, f64, u8) = (600, 7.4, 2);

    let six_hundred = test.0;
    let float_seven = test.1;
    let unsigned = test.2;

    // Array type
    // Every element must be of the same type
    // Fixed length
    // You want the data allocated on the stack
    let a = [1, 2, 3, 4, 5];

    // Useful when you now you will not change the number of elements
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    let a = [3; 5]; // same as let a = [3, 3, 3, 3, 3];

    let first = a[0];
    let second = a[1];
}
