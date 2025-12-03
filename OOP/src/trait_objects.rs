// Trait object approach
// The key differences between trait objects and generics with trait bounds are:
// 1. Trait objects allow for dynamic dispatch, meaning the method to call is determined at runtime.
// 2. Generics with trait bounds use static dispatch, meaning the method to call is determined at compile time.
// 3. Trait objects can hold different types that implement the same trait,
// while generics with trait bounds require all types to be the same.
// 4. Trait objects incur a runtime cost due to dynamic dispatch, while generics with trait bounds do not.


pub trait Draw {
    fn draw(&self);
}

// This approach is in favour of flexibility but has a runtime cost due to dynamic dispatch
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Alternate implementation using generics and trait bounds
pub struct GenericScreen<T: Draw> {
    pub components: Vec<T>,
}

// Here the compiler knows at compile time what type T is, so it can optimize the code better
// There is no runtime cost for dynamic dispatch
impl<T> GenericScreen<T>
// This where clause specifies that T must implement the Draw trait for this impl block to be valid
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button: {}", self.label);
    }
}