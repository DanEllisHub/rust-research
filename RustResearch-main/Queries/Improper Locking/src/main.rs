#[macro_use]
extern crate rocket;
use std::collections::HashMap;
use std::fs;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

const N: usize = 10;

struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: &str, age: u32) -> User {
        User {
            name: name.to_string(),
            age,
        }
    }
}

#[get("/safe_lock_1")]
fn safe_lock_1() -> String {
    let my_mutex = Arc::new(Mutex::new(5));
    print!("1");

    {
        // FIRST SCOPE STARTS HERE
        let mut s = my_mutex.lock().unwrap(); // FIRST LOCK
    } // FIRST SCOPE ENDS HERE - FIRST LOCK RELEASED (SANITIZER)

    {
        // SECOND SCOPE STARTS HERE
        print!("2");
        let mut d = my_mutex.lock().unwrap(); // SECOND LOCK
    } // SECOND SCOPE ENDS HERE - SECOND LOCK RELEASED (SANITIZER)

    {
        // THIRD SCOPE STARTS HERE
        print!("3");
        let mut f = my_mutex.lock().unwrap(); // THIRD LOCK
    } // THIRD SCOPE ENDS HERE - THIRD LOCK RELEASED (SANITIZER)

    "This will print...".to_string()
}

#[get("/safe_lock_2")]
fn safe_lock_2() -> String {
    let my_mutex = Arc::new(Mutex::new(5));
    print!("1");
    let mut s = my_mutex.lock().unwrap(); // SINK - FIRST LOCK
    std::mem::drop(s); // FIRST LOCK RELEASED (SANITIZER)
    print!("2");
    let mut d = my_mutex.lock().unwrap(); // SINK - SECOND LOCK
    std::mem::drop(d); // SECOND LOCK RELEASED (SANITIZER)
    print!("3");
    let mut f = my_mutex.lock().unwrap(); // SINK - THIRD LOCK
    std::mem::drop(f); // THIRD LOCK RELEASED (SANITIZER)

    "This will print...".to_string()
}

#[get("/safe_lock_3")]
fn safe_lock_3() -> String {
    let my_mutex = Arc::new(Mutex::new(5));
    println!("{:?}", my_mutex);
    *my_mutex.lock().unwrap() = 66; // LOCKED AND RELEASED (SINK AND SANITIZER)
    println!("{:?}", my_mutex);
    *my_mutex.lock().unwrap() = 77; // LOCKED AND RELEASED (SINK AND SANITIZER)
    println!("{:?}", my_mutex);
    *my_mutex.lock().unwrap() = 88; // LOCKED AND RELEASED (SINK AND SANITIZER)
    println!("{:?}", my_mutex);
    "This will print...".to_string()
}

#[get("/rw_safe_1")]
fn rw_safe_1() -> String {
    let my_rwlock = RwLock::new(5);
    {
        let read1 = my_rwlock.read().unwrap(); // one .read() is fine
        let read2 = my_rwlock.read().unwrap(); // two .read()s is also fine

        println!("{:?}, {:?}", read1, read2);
    } // Sanitizer - block ends here
    let write1 = my_rwlock.write().unwrap(); // SAFE

    "This will print...".to_string()
}

#[get("/rw_safe_2")]
fn rw_safe_2() -> String {
    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.read().unwrap(); // one .read() is fine
    let read2 = my_rwlock.read().unwrap(); // two .read()s is also fine

    println!("{:?}, {:?}", read1, read2);
    drop(read1); // Releasing the first lock
    drop(read2); // Release the second lock
    let write1 = my_rwlock.write().unwrap(); // SAFE

    "This will print...".to_string()
}
#[get("/rw_safe_3")]
fn rw_safe_3() -> String {
    let my_rwlock = RwLock::new(5);
    let write1 = my_rwlock.write().unwrap(); // First write() is fine
    drop(write1);
    let read1 = my_rwlock.read().unwrap(); // SAFE - write1 is released before reading

    "This will print...".to_string()
}
#[get("/rw_safe_4")]
fn rw_safe_4() -> String {
    let my_rwlock = RwLock::new(5);
    {
        let write1 = my_rwlock.write().unwrap(); // First write() is fine
    } // Sanitizer, write1 is out of scope here
    let read1 = my_rwlock.read().unwrap(); // SAFE - write1 is released before reading

    "This will print...".to_string()
}

#[get("/rw_safe_5")]
fn rw_safe_5() -> String {
    let my_rwlock = RwLock::new(5);
    *my_rwlock.write().unwrap() = 99; // First write() is fine
    let read1 = my_rwlock.read().unwrap(); // SAFE - write1 is released before reading

    "This will print...".to_string()
}

#[get("/rw_deadlock_1")]
fn rw_deadlock_1() -> String {
    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.read().unwrap(); // one .read() is fine
    let read2 = my_rwlock.read().unwrap(); // two .read()s is also fine

    println!("{:?}, {:?}", read1, read2);

    let write1 = my_rwlock.write().unwrap(); // Result - Write after read without relaseing

    "This will not print...".to_string()
}

#[get("/rw_deadlock_2")]
fn rw_deadlock_2() -> String {
    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.read().unwrap(); // one .read() is fine
    let read2 = my_rwlock.read().unwrap(); // two .read()s is also fine

    println!("{:?}, {:?}", read1, read2);

    drop(read2); // Release the second lock
    let write1 = my_rwlock.write().unwrap(); // Result- First lock (read1) is never released

    "This will not print...".to_string()
}

#[get("/rw_deadlock_3")]
fn rw_deadlock_3() -> String {
    let my_rwlock = RwLock::new(5);
    let write1 = my_rwlock.write().unwrap(); // First write() is fine

    let read1 = my_rwlock.read().unwrap(); // RESULT - write1 is never relesaed

    "This will not print...".to_string()
}
#[get("/deadlock_1")]
fn deadlock_1() -> String {
    let my_mutex = Arc::new(Mutex::new(5));
    print!("1");
    let mut s = my_mutex.lock().unwrap(); // SINK - FIRST LOCK
    print!("2");
    let mut d = my_mutex.lock().unwrap(); // RESULT - SECOND LOCK

    "This will never print...".to_string()
}

#[get("/deadlock_2")]
fn deadlock_2() -> String {
    let my_mutex = Arc::new(Mutex::new(5));
    print!("1");
    let mut s = my_mutex.try_lock().unwrap(); // SINK - FIRST LOCK
    print!("2");
    let mut d = my_mutex.lock().unwrap(); // RESULT - SECOND LOCK

    "This will never print...".to_string()
}

#[get("/deadlock_3")]
fn deadlock_3() -> String {
    let my_mutex = Arc::new(Mutex::new(5));
    println!("{:?}", my_mutex);
    let mut s = my_mutex.lock().unwrap(); // SINK - FIRST LOCK
    *s = 66; // NOT A SANITIZER - THIS WON'T RELEASE THE LOCK
    let mut d = my_mutex.lock().unwrap(); // RESULT - SINK (SECOND LOCK WITHOUT RELEASING THE FIRST ONE)
    *d = 77; // WON'T RELEASE THE SECOND LOCK EITHER
    "This will print...".to_string()
}

// #[post("/count?<counter>")]
// fn count(counter : u32) -> String {
//     let balance_mutex = Arc::new(Mutex::new(balance));
//     let mut handle_vec = vec![];

//     for i in 0..credits.len() {
//         let cloned_balance = Arc::clone(&balance_mutex);
//         let handle = std::thread::spawn(move || { // Put the clone in
//             for _ in 0..10 {
//                 *cloned_balance.lock().unwrap() += credits[i];
//             }
//         });
//         handle_vec.push(handle);
//     }

//     handle_vec.into_iter().for_each(|handle| handle.join().unwrap()); // call join on all handles
//     println!("{:?}", balance_mutex);
//     "This will never print...".to_string()
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![safe_lock_1])
        .mount("/", routes![safe_lock_2])
        .mount("/", routes![safe_lock_3])
        .mount("/", routes![deadlock_1])
        .mount("/", routes![deadlock_2])
        .mount("/", routes![deadlock_3])
        .mount("/", routes![rw_deadlock_1])
        .mount("/", routes![rw_deadlock_2])
        .mount("/", routes![rw_deadlock_3])
        .mount("/", routes![rw_safe_1])
        .mount("/", routes![rw_safe_2])
        .mount("/", routes![rw_safe_3])
        .mount("/", routes![rw_safe_4])
        .mount("/", routes![rw_safe_5])
}
