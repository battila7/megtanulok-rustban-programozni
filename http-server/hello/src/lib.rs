use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Result<Self, PoolCreationError> {
        if size == 0 {
            Err(PoolCreationError {})
        } else {
            let mut workers = Vec::with_capacity(size);

            let (sender, receiver) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(receiver));

            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            Ok(ThreadPool {
                workers,
                sender,
            })
        }
    }

    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // Acquiring the lock without assigning to a variable, the temporary MutexGuard
            // returned from the lock method is dropped as soon as the let job statement ends.
            // This ensures that the lock is held during the call to recv, but it is released
            // before the call to job(), allowing multiple requests to be serviced concurrently.
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker {
            id,
            thread,
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to create a new ThreadPool.")
    }
}

impl Error for PoolCreationError {}

