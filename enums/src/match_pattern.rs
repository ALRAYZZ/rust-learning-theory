use crate::match_pattern::UsState::California;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

// Left side patterns to match, right side expressions to return
fn value_in__cents(coin: Coin) -> u8 {
    // Match in rust is EXHAUSTIVE, meaning we must cover all possible cases
    // In this case all 4 variants of the Coin enum
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// To extract and work with the T data inside an Option<T>, we can use a match expression
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some (i + 1), // Where i is the value inside the 'Some' variant
                                        // If Option 'x' passed is a Some variant,
                                        // we take the value inside the 'Some' and add 1 to it
    }
}

pub fn run() {
    let my_coin = Coin::Penny;
    let quarter_coin = Coin::Quarter(California);
    value_in__cents(quarter_coin);
    value_in__cents(my_coin);

    let five = Some(5);

    let result = plus_one(five);
    match result {
        None => println!("No value"),
        Some(i) => println!("The value is: {}", i),
    }

    let dice_roll = 9;
    // We can use 'other' as a catch-all pattern to match any value not specifically listed
    // This is called a 'catch-all' pattern
    match dice_roll {
        7 => remove_fancy_hat(),
        3 => add_fancy_hat(),
        other => move_player(other), // catch-all pattern must run last
        _ => reroll(), // another catch-all pattern but this time the '_' indicates we don't care about the value
        _ => (), // same as above but does nothing
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}
}