fn run() {
    let config_max = Some(3u8); // This means integer 3 of type unsigned 8-bit


    // A match pattern that only does something in one case is an 'IF' pattern potentially
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Here the matching is the 'Some' type, and the 'if let' syntax allows us to concisely handle that case
    // So if config_max is a 'Some' variant, we bind the value inside to 'max' and execute the block
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}