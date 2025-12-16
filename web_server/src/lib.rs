use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

// ThreadPool manages a fixed number of worker threads that share work from a queue
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>, // Sending end of channel to dispatch jobs to workers
}

// Job is a closure that can be executed once, sent across threads, and has a static lifetime
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // mpsc = multiple producer, single consumer channel for thread communication
        let (sender, receiver) = mpsc::channel();

        // Arc = Atomic Reference Counting for shared ownership across threads
        // Mutex = Mutual exclusion lock to ensure only one thread accesses receiver at a time
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        // Spawn worker threads, each gets a cloned reference to the shared receiver
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f); // Box allocates closure on heap (required for trait objects)

        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); // Close the sending end to signal workers to stop
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // Gracefully terminate each worker thread
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}