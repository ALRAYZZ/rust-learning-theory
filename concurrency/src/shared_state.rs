// Difference between channels and mutex is that channels are for communication between threads
// while mutexes are for shared state within threads
// We use channels when we want to send data from one thread to another
// We use mutexes when we want to share data between multiple threads safely

// Channels are ideal for scenarios where you want to decouple the producer and consumer of data,
// allowing them to operate independently.

// Mutexes are better suited for scenarios where multiple threads need to access and modify shared data,
// ensuring that only one thread can access the data at a time to prevent data races
// Like a shared counter that multiple threads incrementing its value.

use std::sync::{Arc, Mutex};
use std::thread;
// API of mutex
pub fn run() {
    // Here we create a Mutex that will allow safe shared access to the data it protects
    let m = Mutex::new(5);

    { // Start a new scope to limit the lifetime of the lock
        let mut num = m.lock().unwrap(); // Lock the mutex to gain access to the data
        *num = 6; // Modify the data inside the mutex
    } // The lock is automatically released here when num goes out of scope

    // Now we can print the modified data
    println!("m = {:?}", m);

    shared_mutex_between_threads();
}

// Sharing a Mutex<T> between multiple threads

// Rust single ownership model clashes with the idea of sharing data between multiple threads
// We need a way for multiple threads to own a piece of the Mutex without violating Rust's ownership rules

// So Arc<T> (Atomic Reference Counted) is a smart pointer that enables multiple ownership of the same data
// across different threads. It keeps track of the number of references to the data,
// and when the last reference goes out of scope, it automatically cleans up the data.
fn shared_mutex_between_threads() {
    // Create a Mutex to protect a shared counter
    // We use Arc to allow multiple ownership of the Mutex across threads else,
    // the Mutex would be owned by only one thread, and we couldn't share it
    let counter = Arc::new(Mutex::new(0));

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    // Create 10 threads that will each increment the counter
    for _ in 0..10 {
        // Clone the Arc to get a new reference to the same Mutex for each thread
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // Lock the mutex to gain access to the counter for the specific thread
            let mut num = counter.lock().unwrap();

            *num += 1;
        }); // Closure ends here so the lock is released when the thread finishes

        // For each thread we create, we store its handle in the handles vector so we can join them later
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}