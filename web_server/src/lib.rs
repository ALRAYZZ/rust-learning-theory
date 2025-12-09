use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

// ThreadPool manages a fixed number of worker threads that share work from a queue
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>, // Sending end of channel to dispatch jobs to workers
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

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f); // Box allocates closure on heap (required for trait objects)

        self.sender.send(job).unwrap(); // Send job to channel, any available worker will pick it up
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // lock() acquires mutex, recv() blocks until a job arrives
            // This pattern ensures only one worker processes each job
            let job = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();

            println!("Worker {id} got a job; executing.");

            job(); // Execute the closure
        });

        Worker {id, thread}
    }
}