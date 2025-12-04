// Matching literals
fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// Matching named variables
fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    // This introduces a problem called shadowing.
    // The pattern x in the match arm Some(x) shadows the outer x variable.
    // To fix this, we can use a different name in the pattern.
    match x {
        Some(50) =>println!("Got 50"),
        // Here y is shadowing the outer y variable. This means that inside this match arm,
        // y refers to the value inside Some, not the outer y variable.
        // So it becomes a binding pattern since Some(y) matches Some(value) and binds value to y.
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

// Matching multiple patterns
fn matching_multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}


// Matching ranges of values with ..=
fn matching_ranges() {
    let x = 5;

    // If x is between 1 and 5 inclusive, this arm will match.
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something"),
    }

    // Matching char ranges
    let c = 'c';
    match c {
        'a'..='j' => println!("early alphabet"),
        'k'..='z' => println!("late alphabet"),
        _ => println!("something else"),
    }
}


// We can also use patterns to destructure structs, enums, and tuples.
// Destructuring is purely for extracting values from complex data types. It is NOT
// a mechanism for modifying the original struct or enum instances.
fn destructuring_structs() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point  {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

// Destructuring enums
fn destructuring_enums() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("ChangeColor to ({r}, {g}, {b})"),
    }
}