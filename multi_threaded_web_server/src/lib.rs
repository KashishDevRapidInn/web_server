use std::thread;
use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool{
    // thread pool will handle the connection and not return anything, so T will be the unit type ().
    // threads: Vec<thread::JoinHandle<()>>,

    workers: Vec<Worker>,
    sender: mpsc::Sender<Messages>,
}


impl ThreadPool{
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size:usize)->ThreadPool{
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver=  Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(id,  Arc::clone(&receiver)));
        }
        ThreadPool{workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        // FnOnce because the thread for running a request will only execute that request’s closure one time, which matches the Once in FnOnce.
        // Send to transfer the closure from one thread to another
        // 'static because we don’t know how long the thread will take to execute.
    {
        let job = Box::new(f);
        self.sender.send(Messages::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending Terminate message to all workers");

        for _ in &self.workers{
            self.sender.send(Messages::Terminate).unwrap();
        }
          println!("Shutting down all workers");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


// thread::spawn expects to get some code the thread should run as soon as the thread is created. However, in our case, we want to create the threads and have them wait for code that we’ll send later. The standard library’s implementation of threads doesn’t include any way to do that; we have to implement it manually. Therefore we create a Worker Struct

struct Worker {
    id: usize,
    // thread: thread::JoinHandle<()>,
    thread: Option<thread::JoinHandle<()>>,
    // Why This Works
    // Option Type: Allows you to optionally hold a value, in this case, the JoinHandle. Using Option lets you take ownership of the JoinHandle by moving it out, without affecting the rest of the Worker struct.
    // take Method: Moves the value out of the Option, replacing it with None. This method provides ownership of the JoinHandle so it can be joined.
}

impl Worker {
    fn new(id: usize, receiver:  Arc<Mutex<mpsc::Receiver<Messages>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();


            match message{
                Messages::NewJob(job)=>{
                    println!("Worker {} got a job, executing", id);
                }
                Messages::Terminate=>{
                    println!("Worker {} has told to terminate", id);
                    break;
                }
            }
            // println!("Worker {id} got a job; executing.");

            // job();
        });
        Worker { id,  thread: Some(thread), }
    }
}

enum Messages{
    NewJob(Job),
    Terminate
}