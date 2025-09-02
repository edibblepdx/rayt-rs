// Assign threads jobs by scan line.
//
// possibly have to move file pointer offset for each write.
// This should still be faster than single threaded math.
//
// Can either have a queue in the main thread or a shared
// file descriptor within a mutex.
//
// Maybe implement a writer class or trait with several* image
// formats. I'll probably just stick to ppm though to be honest.
// But the ImageWriter trait could have a write(x, Some(offset))
// method. And it should have a mutex possibly. Then implement
// ImageWriter for formats/ppm.rs or something. That leaves it
// open for future image types even though I'll likely never
// actually implement them. Maybe I could implement std::io::Write
//
// I could have a ppm struct with a file descriptor mutex and
// implement the Write trait. I have no idea what flush does.
// And am not sure how to handle the offset. Maybe just a method
// on the struct? I need to be careful not to lock, move the
// pointer, and unlock without writing though.

extern crate num_cpus;

use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

use log::info;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

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
        assert!(size > 0, "nthreads must be greater than zero");

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, rx.clone()));
        }

        ThreadPool {
            workers,
            sender: Some(tx),
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

/// Create a thread pool with one thread per CPU.
impl Default for ThreadPool {
    fn default() -> ThreadPool {
        ThreadPool::new(num_cpus::get())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers.drain(..) {
            info!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        info!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        info!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}
