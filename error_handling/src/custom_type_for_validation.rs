// We can use Rust type system to ensure we have valid values
use std::io;

fn run() {
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // The if expression checks if the guess is within the valid range
        if guess < 1 || guess > 100 {
            println!("Please enter a number between 1 and 100: ");
            continue;
        }

        match guess.cmp(&secret_number) {
            ////
        }
    }
}

// Instead we can make a new type and put the validation in a function to create an instance
// of the type with already a valid value

pub struct Guess {
    value: i32,
}

impl Guess {
    // This method has the constructor logic Guess { value } but after doing validation
    // So only the new method can create an instance of Guess
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        // Constructor returns an instance of Guess with a valid value
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
