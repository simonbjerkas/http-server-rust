mod error;
mod request;

pub use request::{Protocol, Request, StatusCode};

use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

use anyhow::Result;
use error::ServerError;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new threadpool
    ///
    /// the size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// the `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    /// Custom destructor for threadpool
    ///
    /// # Panics
    ///
    /// the `drop` function will panic if the worker fails to join, or the woker got poisoned when running
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker
                .thread
                .join()
                .expect("worker paiced during drop")
                .unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<Result<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || -> Result<()> {
            loop {
                let job = receiver
                    .lock()
                    .map_err(|_| ServerError::PoisonedWorker(id))?
                    .recv();

                match job {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down");
                        break;
                    }
                }
            }
            Ok(())
        });

        Worker { id, thread }
    }
}
