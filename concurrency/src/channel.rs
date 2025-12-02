// A channel is a way to communicate between threads, we can send data from one thread to another
// mpsc = multiple producer, single consumer means that we can have multiple threads
// sending data to the same channel, but only one thread receiving data from it.


use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread and use move to transfer ownership of tx to the new thread
    thread::spawn(move || {
        let val = String::from("hi");
        // This uses the tx sender to send the value to the receiver
        tx.send(val).unwrap();

        // Compiler will not allow this print because val has been moved its ownership
        // to the channel when we called tx.send(val) now rx owns it
        //println!("val is {val}");
    });

    // The main thread waits here to receive the value from the channel
    // We could also ouse try_recv to not block the main thread so it can do other work
    let received = rx.recv().unwrap();
    println!("Got: {received}");


    println!("\nNow calling sending_multiple_values:\n");
    sending_multiple_values();

    println!("\nNow calling multiple_producers:\n");
    multiple_producers();
}



// Tx is owned by the spawned thread, and Rx is owned by the main thread
// Here we will send multiple values from one thread to another
pub fn sending_multiple_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
       let vals = vec![
           String::from("hi"),
           String::from("from"),
           String::from("the"),
           String::from("thread"),
       ];

        // For every item in vector vals send it to the channel and sleep for 1 second
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Just sending does nothing, so here we use the receiver stream to get
    // each value as it comes in and print it
    // Also here we treat rx as an iterator, so it will keep receiving values
    // This loop will end when the sender is dropped (when the spawned thread ends)
    for received in rx {
        println!("Got: {received}");
    }
}

// We can also have multiple producers sending to the same channel
// The main issue with this implementation is that we do not control the order
// in which the messages are received, since both threads are sending messages
// at the same time, so the output will be interleaved.
// Everytime we run the program we may get a different order of messages.
pub fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("first"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap(); // using tx1 which is the clone of tx
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("second"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap(); // using the original tx
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
