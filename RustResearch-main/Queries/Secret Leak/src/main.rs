#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};
use ring::aead::AES_256_GCM;
use ring::rand;
use ring::rand::SecureRandom;
//use hexupper from data-encoding
use data_encoding::HEXUPPER;

pub fn save_password(id: &str, password: &str) -> Result<(),()>{
    /*saves password to db */
    Ok(())
}



//vulnerable code flow 1
#[get("/generate_key")]
fn generate_key() -> String { 
    /* Generate a new symmetric encryption key */    
    let rand = rand::SystemRandom::new();
    let mut key_bytes = vec![0; AES_256_GCM.key_len()];
    rand.fill(&mut key_bytes);
    format!("key_bytes = {}", HEXUPPER.encode(&key_bytes))
}


//vulnerable code flow 2
#[get("/update_password?<id>&<password>")]
fn update_password(id: String, password: String) -> String { 
    /* save password to DB */
    match save_password(&id, &password){
        Ok(()) => format!("Password updated for user {}. New password is {}", id, password),
        Err(e) => format!("Error updating password {:?}", e),
    }
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/update_password", routes![update_password])
}