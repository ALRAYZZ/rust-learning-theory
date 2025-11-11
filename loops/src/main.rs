fn main() {
    let mut counter = 0;
    let test = 1;

    //test = 2; // This line will cause a compile-time error because `test` is immutable.

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    let mut count = 0;
    // labeled loop example
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    println!("The result is: {result}");

    while_loop_example();
    for_loop_example();
    for_loop_range_example();
}

fn while_loop_example() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
}

fn for_loop_example() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
}

fn for_loop_range_example() {
    // Exclusive range from 1 to 3
    // the .rev() method reverses the range so starts at 3 and goes down to 1
    for number in (1..4).rev() {
        println!("{number}!");
    }

    for number2 in 1..=10 {
        println!("{number2}!");
    }
    println!("LIFTOFF!!!");
}
