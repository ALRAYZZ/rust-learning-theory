mod refutability;

fn main() {
}


// Match expression patterns with Option type
// Match expressions need to be exhaustive, meaning all possible cases must be handled.
// we have the option to use a wildcard pattern (_) to catch all other cases.
fn match_expression(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


// This is a function that demonstrates the use of if let pattern matching
// to handle multiple conditions in a more concise way than using match expressions.
// Compiler does not require exhaustiveness for if let statements.
// Potentially logic bug for unhandled cases.
fn if_let_pattern() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // In this if let, we are checking, first, if the user has a favorite color. Type Some(&str)
    // If so, we use that color as the background.
    // So we are saying If favorite_color is a type Some(&str),
    // then bind the value inside Some variant to the variable color.
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

// This pattern allows the while loop to continue executing as long as the pattern matches.
fn while_let_pattern() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // While we can pop a value from the stack, meaning .pop method can return Some(value),
    // we bind that value to the variable top and execute the body of the loop.
    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}

// For loops are also pattern matching.
fn for_loop_pattern() {
    let v = vec!['a', 'b', 'c'];

    // Here we are breaking a vector into an index and a value using the .iter().enumerate() method,
    // which returns an iterator that yields pairs of index and value.
    // Here the pattern matching is done implicitly by the for loop.
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

}

// Even simple let statements are pattern matching.
fn let_statement_pattern() {
    // This is a simple let statement that binds the value 5 to the variable x.
    // let PATTERN = EXPRESSION;
    let x = 5;

    // We see the pattern is 3 variables (x, y, z) being matched to the expression (1, 2, 3).
    let (x, y, z) = (1, 2, 3);

    // This won't compile because the pattern (x, y) does not match the expression (1, 2, 3)
    let (x, y) = (1, 2, 3);
}