use std::{sync::mpsc, thread, time::Duration};

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

// MPSC is a way to send data between threads in a concurrent program.
// MPSC stands for Multiple Producer, Single Consumer.
// Single thread can receive data, while multiple threads can send data.
// Is the standard Rust solution for safe, asynchronous inter-thread communication.
// Solves data races and mutability issues by enforcing ownership rules at compile time.
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    // TODO: We want to send `tx` to both threads. But currently, it is moved

    //Clone the Sender BEFORE the original `tx` is moved into the first closure.
    // This creates a second, independent handle for the same channel.
    // So both threads can send messages concurrently.
    let tx_clone = tx.clone();

    // Thread 1: Takes ownership of the ORIGINAL `tx` handle and the first half of the queue.
    thread::spawn(move || {
        for val in q.first_half {
            println!("Sending {val:?}");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
    // Thread 2: Takes ownership of the CLONED `tx` handle and the second half of the queue.
    thread::spawn(move || {
        for val in q.second_half {
            println!("Sending {val:?}");
            tx_clone.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
    // All this is the right pattern for MPSC in Rust.
    // In this case we create 2 threads producing data concurrently,
    // while the main thread consumes that data.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
