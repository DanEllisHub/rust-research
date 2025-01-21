use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::sync::mpsc::channel;

static mut COUNTER: i32 = 0;

fn main() {
    let mut handles = vec![];
    for _ in 0..10 {
        // Sink - The thread being created in unsafe way
        handles.push(thread::spawn(|| unsafe {
            for _ in 0..1000 {
                COUNTER += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    // Not a sink - Unsafe block using the data
    unsafe {
        println!("Final counter: {}", COUNTER);
    }
}

fn fn2() {
    let data = vec![1, 2, 3, 4];
    let idx = Arc::new(AtomicUsize::new(0));
    let other_idx = idx.clone();

    // `move` captures other_idx by-value, moving it into this thread
    thread::spawn(move || {
        // It's ok to mutate idx because this value
        // is an atomic, so it can't cause a Data Race.
        other_idx.fetch_add(10, Ordering::SeqCst);
    });

    if idx.load(Ordering::SeqCst) < data.len() {
        // Sink
        unsafe {
            // Incorrectly loading the idx after we did the bounds check.
            // It could have changed. This is a race condition, *and dangerous*
            // because we decided to do `get_unchecked`, which is `unsafe`.
            println!("{}", data.get_unchecked(idx.load(Ordering::SeqCst)));
        }
    }
}




fn Send_Vulnerable() {
    let (tx, rx) = channel();
    // Node 1 - The thread being created
    thread::spawn(move || {
        // Node 2 - Send method()
        tx.send("Hello, world!").unwrap();
    });
    // Sink - try_recv()
    if let Ok(message) = rx.try_recv() {
        println!("{}", message);
    }
}