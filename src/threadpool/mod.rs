use std::sync::{Arc, Mutex, mpsc};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool { 
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // Wrap receiver in Arc + Mutex so multiple workers can use it safely
        let receiver = Arc::new(Mutex::new(receiver));

        // Instantiating workers 
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // Each worker gets a reference to the *same* receiver.
            workers.push(Worker::new(id, Arc::clone(&receiver)));
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
            sender.send(job).unwrap();
        } else {
            panic!("Sender has been dropped, ThreadPool will shut down — cannot execute more jobs");
        }
    }
}

impl Drop for ThreadPool { 
    fn drop(&mut self) {
        println!("Threadpool dropping, closing sender...");

        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    println!("Worker {id} got a job, executing.");
                    job()
                },
                Err(_) => {
                    println!("Worker shutting down.");
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