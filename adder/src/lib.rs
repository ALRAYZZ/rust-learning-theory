pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    // This macro creates a formatted string
    // so we can include the name in the greeting
    format!("Hello {name}!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "Guess value must be between 1 and 100, got {}.",
                value
            );
        }
        Guess { value }
    }
}

// This module is only compiled when running tests
// Contains test functions that will run with `cargo test`
#[cfg(test)]
mod tests {
    use super::*; // Import all items from the parent module

    #[test] // Indicates that this is a test function
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4); // Asserts that one is equal to the other
    }

    //#[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // Assert macro checks that the expression returns true
        // assert (this is true) else fail
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // Assert macro checks that the expression is true
        // In this case we use ! to invert the boolean value
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        // Assert that 4 == add_two(2) else fail
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // We add a 2nd parameter to assert! to customize the failure message
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );

    }

    #[test]
    #[should_panic] // This test should panic to pass
    fn greater_than_100() {
        Guess::new(200);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}