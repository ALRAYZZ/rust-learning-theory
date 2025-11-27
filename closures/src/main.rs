// A closure is an anonymous function that can capture variables from the surrounding scope.
// A function
// Create on fly
// Can remember variables where it was created
// Can be stored in a variable or passed as an argument to other functions

// WHY USE CLOSURES?
// 1. Concise syntax for small functions
// 2. Passing behavior to functions
// 3. Can use local variables without needing to pass them as parameters


// WHEN USE CLOSURES?
// 1. Iterator chains
// 2. Callbacks
// 3. Custom sorting or filtering logic
// 4. Configurable functions
// 5. Functional programming patterns (mapping, filtering, reducing)


fn run() {
    let x = 4;
    let y = || println!("{}", x); // closure captures x by reference
    y();                               // prints 4 (x)
}

mod caputring_moving_ownership;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // || self.most_stocked() is a closure that takes no parameters
    // and returns the result of self.most_stocked()
    fn giveaway (&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    // This function gets called by the closure above if the user has no preference
    // It just takes as parameter the reference to the Inventory struct we instanced
    // on main and counts which color is more stocked
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);
}


