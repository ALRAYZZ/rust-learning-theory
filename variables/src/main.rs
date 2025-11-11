fn main() {
    let x = 3;

    println!("First X {x}");

    let x = x + 2;
    println!("Second X {x}");

    {
        let x = x * 3;
        println!("Value of inner x {x}");
    }

    // Mutable variable cannot change type
    let mut y = 5;
    y = 3;
    y = "hello";

    let spaces = "   ";
    let spaces = spaces.len();

    println!("Spaces of {spaces}");

    println!("Third X {x}");
}
