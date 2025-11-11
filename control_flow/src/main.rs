fn main() {
    let number = 3;
    is_true(number);

    let divisible = 6;

    if divisible % 4 == 0 {
        println!("divisible by 4");
    } else if divisible % 3 == 0 {
        println!("divisible by 3");
    } else if divisible % 2 == 0 {
        println!("divisible by 2");
    } else {
        println!("not divisible by 4, 3, or 2");
    }


    let condition = false;
    // Using if as an expression to assign a value to a variable
    let number_conditional = if condition { 5 } else { 10 };
    println!("The value of numConditional is: {number_conditional}");
}


fn is_true(param: i32) -> bool {
    if param < 5 {
        println!("condition was true");
        true
    } else {
        println!("condition was false");
        false
    }
}
