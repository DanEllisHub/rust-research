use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;

fn main() {
    
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    for _ in 0..3 {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            // Sanitizer
            let mut data = data.lock().unwrap();
            data[0] += 1;
        });
    }
}

fn func2() {
    let counter = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // Node 1 - thread creation
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                // Sanitizer
                let mut num = counter.write().unwrap();
                *num += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", *counter.read().unwrap());
}



fn func3() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        tx.send("Hello, world!").unwrap();
    });
    
    // Sanitizer 
    if let Ok(message) = rx.recv() {
        println!("{}", message);
    }
}