use std::sync::{Arc, Mutex};
use crossbeam_channel::{unbounded, Sender, Receiver};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool { 
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = unbounded::<Job>();

        // Instantiating workers 
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // Each worker gets a reference to the *same* receiver.
            let worker_receiver = receiver.clone();
            workers.push(Worker::new(id, worker_receiver));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    // Box and send a job to a worker through the sender object.
    pub fn execute<F>(&self, f: F)
    where
        // F is a generic type used in this function to define the kind of function it receives as parameter.
        F: FnOnce() + Send + 'static, // Send means it can go to other threads, and 'static means it owns all its values (no borrowing)
    {
        // A box is a smart pointer to something in heap memory, in this case, a function.
        let job = Box::new(f);
        
        // Make sure sender still exists.
        if let Some(sender) = &self.sender {
            sender.send(job).expect("Threadpool receiver dropped.");
        } else {
            panic!("Sender has been dropped — cannot execute more jobs");
        }
    }
}

impl Drop for ThreadPool { 
    fn drop(&mut self) {
        println!("Threadpool dropping, closing sender...");

        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap(); 
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Receiver<Job>) -> Worker {
        let thread = std::thread::spawn(move || {
            // 'iter()' blocks untill a job arrives or the channel is closed
            for job in receiver {
                println!("Worker {} got a job, executing...", id);
                job();
            }
            println!("Worker {} shutting down...", id);
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}