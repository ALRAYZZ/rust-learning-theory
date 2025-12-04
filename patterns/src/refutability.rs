// Patterns in Rust can be either refutable or irrefutable.
// An irrefutable pattern is one that will always successfully match a value of the appropriate type
// For example, in a let statement, the pattern must be irrefutable because it has to match
// the value being assigned.
// Examples of irrefutable patterns include:
// - A variable name (e.g., x)
// - A tuple with fixed elements (e.g., (x, y))
// - A struct with all fields specified (e.g., Point { x, y })
// On the other hand, a refutable pattern is one that may not match a value.
// Refutable patterns are typically used in constructs like if let, while let, and match statements,
// where the program needs to handle cases where the pattern does not match.
// Examples of refutable patterns include:
// - An enum variant (e.g., Some(x), None)
// - A tuple with optional elements (e.g., (x, Some(y)))
// - A struct with optional fields (e.g., Point { x, y: Some(y) })
// Understanding the distinction between refutable and irrefutable patterns is important
// for writing correct and efficient Rust code, as it affects how patterns can be used
// in different contexts.



// Examples of what happens when we try to use a refutable pattern where
// Rust expects an irrefutable one:


fn run() {
    // This is incorrect because let statements require irrefutable patterns.
    // and Some(x) is a refutable pattern, because Option can be None or Some(value).
    let Some(x) = some_option_value;

    // If we have refutable pattern where an irrefutable one is expected,
    // we can fix it by changing the code that uses the pattern: instead of using let,
    // we can use if let to handle the refutable pattern correctly.
    if let Some(x) = some_option_value {
        println!("{x}"); // if pattern does not match, this block will be skipped
    }

    // Here compiler will because does not make sense to use if let with an irrefutable pattern.
    if let x = 5 {
        println!("{x}");
    }
}