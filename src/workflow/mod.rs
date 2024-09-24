use crossbeam::queue::SegQueue;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
    time::Duration,
};

enum Job {
    Task(Box<dyn FnOnce() -> Result<(), Box<dyn std::error::Error>> + Send + 'static>),
    Shutdown,
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        job_queue: Arc<SegQueue<Job>>,
        job_signal: Arc<(Mutex<bool>, Condvar)>,
        running: Arc<AtomicBool>,
    ) -> Self {
        let thread = thread::spawn(move || loop {
            match job_queue.pop() {
                Some(Job::Task(task)) => if task().is_err() {},
                Some(Job::Shutdown) => {
                    break;
                }
                None => {
                    let (lock, cvar) = &*job_signal;
                    let mut available = lock.lock().unwrap();

                    while !*available && running.load(Ordering::Relaxed) {
                        available = cvar
                            .wait_timeout(available, Duration::from_millis(100))
                            .unwrap()
                            .0;
                    }
                    *available = false;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
