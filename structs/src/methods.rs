#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Everything within impl (implementation) block is associated with the Rectangle struct
// Associated functions
// We can have multiple impl blocks for the same struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // using same method name as field name
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions without &self parameter are not methods, and usually used as CONSTRUCTORS
    fn square(size: u32) -> Self {  // Self is an alias for the type we are implementing, here Rectangle
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn run() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Since no self in square function, we call it using the struct name and using the :: syntax
    let sq = Rectangle::square(30);

    println!("{}", rect1.width());

    // Then we can call the method using dot notation
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}