mod channel; // Import the channel module

use std::thread;
use std::time::Duration;



fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // By calling the join here, we have more control over when the main thread waits
    // for the spawned thread to finish.
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // Blocks the main thread for the spawned thread to finish by making it a variable and calling join
    //handle.join().unwrap();

    move_closures_threads();

    channel::run();
}


fn move_closures_threads() {
    let v = vec![1, 2, 3];

    // By using the move keyword, we transfer the ownership of v to the closure
    // so rust now knows that v will be used in the new thread, and is not worried
    // about the reference remaining valid after the main thread ends.
    let handle = thread::spawn(move || {
       println!("Here is a vector: {:?}", v);
    });

    handle.join().unwrap();
}
