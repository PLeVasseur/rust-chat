use crossbeam_channel::{unbounded, Receiver};
use std::thread;
use std::thread::JoinHandle;

// Note the type alias is possible
type Alphabet = String;

// Note that we can decorate an enum with a #[derive(...)] to
// get automatic implementations of traits
// It's possible to write your own as well as a procedural macro (advanced)
#[derive(Debug)]
enum Task {
    Print(String), // Note how in Rust enums each member can hold different data
    Compute(u32, u32),
    Exit,
    // Note how when we add this additional element the Rust compiler complains since we didn't handle it in the match
    OneMoreThing(Alphabet),
}

// Note how we define a struct
struct Worker {
    join_handle: JoinHandle<()>,
    worker_id: usize,
}

// Note how we implement functions off of structs
impl Worker {
    pub fn new(join_handle: JoinHandle<()>, worker_id: usize) -> Self {
        Self {
            join_handle,
            worker_id,
        }
    }
}

fn worker(id: usize, receiver: Receiver<Task>) {
    for task in receiver {
        println!("Processing task: {:?}", task);
        // Note how we can do pattern matching
        match task {
            // and destructure the enum into its composite elements
            Task::Print(message) => println!("Worker {}: {}", id, message),
            Task::Compute(a, b) => println!("Worker {}: {} + {} = {}", id, a, b, a + b),
            Task::Exit => {
                println!("Worker {} exiting.", id);
                break;
            }
            // Note how we can insert variables directly into strings we're printing
            Task::OneMoreThing(alphabet) => {
                println!("Worker {id} singing the alphabet: {alphabet}")
            }
        }
    }
}

fn main() {
    let (sender, receiver) = unbounded();

    let mut workers = vec![];
    for id in 1..4 {
        let receiver = receiver.clone();
        workers.push(Worker::new(thread::spawn(move || worker(id, receiver)), id));
    }

    // Note how we can use .unwrap() or .expect() in order to discard errors and panic if they occur
    sender.send(Task::Print("Hello, world!".into())).unwrap();
    sender.send(Task::Compute(42, 27)).unwrap();
    sender.send(Task::Print("Rust is awesome!".into())).unwrap();
    // Note how we can do error handling to handle these error conditions explicitly and it's visible
    if let Err(err) = sender.send(Task::OneMoreThing(
        "A B C D ... Next time won't you sing with me!".into(),
    )) {
        println!("Had an error while sending! err: {err:?}");
    }

    for _worker in &workers {
        // Note how the loop takes the variable immutable by default so that we are unable to mutate
        // it within the thread
        // worker.worker_id = 10;
        sender.send(Task::Exit).unwrap();
    }

    // Wait for all workers to finish
    for worker in workers {
        println!("Stopping worker_id: {}", worker.worker_id);
        worker.join_handle.join().unwrap();
    }
}
