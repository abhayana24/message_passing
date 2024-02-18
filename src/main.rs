use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

struct Task {
    id: u32,
    payload: String
}

struct Worker {
    id: u32
}

impl Task {
    fn new(id: u32, payload: String) -> Task {
        Task {
            id,
            payload
        }
    }
}

impl Worker {
    fn new(id: u32) -> Worker {
        Worker {
            id
        }
    }

    fn process_task(&self, task: Task){
        println!("Worker {} is processing task {} with payload {}", self.id, task.id, task.payload)
    }
}

fn create_task(id: u32, payload: String) -> Task {
    Task::new(id, payload)
}

fn create_worker(id: u32) -> Worker {
    Worker::new(id)
}

fn main() {
    // tx is sender handler, rx is receiver handler
    let (tx, rx) = mpsc::channel();

    // rx handler is wrapped in Arc and Mutex to be shared between threads safely 
    let rx = Arc::new(Mutex::new(rx));

    let num_workers = 4;

    let mut handles = vec![];

    //create vector of 1000 tasks in loop to be distibuted among worker threads
    let tasks: Vec<Task> = (0..1000).map(|i| create_task(i, format!("task {}", i))).collect();

    //distributing vector of tasks defined above among worker threads, we should ensure main thread waits for all worker threads to finish their work before it exits and display a cmplettion message
    for id in 0..num_workers {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || {
                loop {
                    let task = rx.lock().unwrap().recv_timeout(Duration::from_secs(5));
                    match task {
                        Ok(task) => {
                            let worker = create_worker(id);
                            worker.process_task(task);
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => {
                            println!("Worker {} timed out waiting for tasks", thread::current().name().unwrap_or("unnamed"));
                            continue;
                        }
                        Err(mpsc::RecvTimeoutError::Disconnected) => break
                    }
                }
        });
        handles.push(handle);
    }



    for task in tasks {
        tx.send(task).unwrap();
    }

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All tasks have been processed successfully!")

}


