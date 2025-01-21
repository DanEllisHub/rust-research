#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};
use ring::aead::AES_256_GCM;
use ring::rand;
use ring::rand::SecureRandom;
use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use data_encoding::HEXUPPER;




//vulnerable code
#[get("/allocate_mem?<bytes>")]
fn allocate_mem(bytes: usize) { 
    let layout = Layout::from_size_align(bytes, 1).unwrap();
    let ptr = unsafe { alloc(layout) }; //Sink
}


//vulnerable code
#[get("/allocate_vec?<bytes>")]
fn allocate_vec(bytes: usize) { 
    let key_value = vec![0; bytes]; //Sink
    let vec: Vec<i32> = Vec::with_capacity(bytes); //Sink
}
 //safe code
 #[get("/allocate_safe?<bytes>")]
fn allocate_safe(bytes: usize) { 
    if bytes < 1024{
        let key_value = vec![0; bytes]; //Sink
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/allocate_mem", routes![allocate_mem])
        .mount("/allocate_vec", routes![allocate_vec])
        .mount("/allocate_safe", routes![allocate_safe])
}