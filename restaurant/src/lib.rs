use std::fmt::Result;
use std::io::Result as IoResult; // using 'as' to create an alias for Result type from io module


fn deliver_order() {}

mod back_of_house { //  so back of house is a child of the root module in this case
    pub struct Breakfast {
        pub toast: String, // this can be accessed from outside the module
        seasonal_fruit: String, // this cannot be accessed from outside the module
    }

    // Is standard to make enums public, else it's not convenient to declare all variants as public
    pub enum Appetizer {
        Soup,
        Salad, // both variants are public because the enum is public
    }

    impl Breakfast {
        // because Breakfast has a private field,
        // we need to provide a public associated function to construct it
        // else we wouldn't be able to create an instance of Breakfast from outside this module
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // using super to call function in parent module
    }
    fn cook_order() {}
}

mod front_of_house {
   pub mod hosting {
       pub fn add_to_waitlist() {} // we need to make the function public to access it outside the module

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}


use crate::front_of_house::hosting;

pub fn eat_at_restaurant () {
    // Absolute path (after the use statement we can remove crate::front_of_house)
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // This line won't compile because seasonal_fruit is private
    // meal.seasonal_fruit = String::from("blueberries");

    // enum is public so we can use it here
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
