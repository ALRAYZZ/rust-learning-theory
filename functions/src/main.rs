fn main() {
    other_function(5);
    print_labeled_measurement(5, 'm');

    // statements are instructions that perform some action and do not return a value
    let y = 6;

    // expressions evaluate to a value
    // the result of the expression gets assigned to x
    let x = {
        let y = 3;
        y + 1 // No semicolon here, because it's an expression, if semicolon is added it becomes a statement and returns nothing
    };
    println!("The value of x is: {x}");

    // functions can also return values
    let b = five();
    println!("The value of b is: {b}");

    let c = plus_one(b);
}

// In rust functions must declare type of parameters
fn other_function(x: i32) {
    println!("The value of x: {x}");
}


fn print_labeled_measurement(value: i32, uint_label: char) {
    println!("The measurement is: {value}{uint_label}");
}

fn five() -> i32 {
    5 // no semicolon, so it's an expression that returns 5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // no semicolon, so it's an expression that returns x + 1
}
