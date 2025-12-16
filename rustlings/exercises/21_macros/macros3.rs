// TODO: Fix the compiler error without taking the macro definition out of this
// module.
mod macros {
    // This decorator makes the macro available outside of this module to the crate root
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
