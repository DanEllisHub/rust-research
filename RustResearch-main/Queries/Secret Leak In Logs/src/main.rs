#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};
use ring::aead::AES_256_GCM;
use ring::rand;
use ring::rand::SecureRandom;
use data_encoding::HEXUPPER;
use log::{info, warn};

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
    info!("Generated new key: {}", HEXUPPER.encode(&key_bytes)); //Sink
    return "Password updated successfully".to_string();
}


//vulnerable code flow 2
#[get("/update_password?<id>&<password>")]
fn update_password(id: String, password: String) -> String { 
    /* save password to DB */
    match save_password(&id, &password){
        Ok(()) => {
            warn!("Password {} for user {} updated successfuly",id,password); //Sink
            return "Password updated successfully".to_string()},
        Err(e) => format!("Error updating password {:?}", e),
    }
    
}

//safe 
#[get("/safe_update_password?<id>&<password>")]
fn safe_update_password(id: String, password: String) -> String { 
    /* save password to DB */
    match save_password(&id, &password){
        Ok(()) => {
            warn!("Password for user {} updated successfuly",id);
            return "Password updated successfully".to_string()},
        Err(e) => format!("Error updating password {:?}", e),
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/update_password", routes![update_password])
        .mount("/[generate_key", routes![generate_key])
        .mount("/safe_update_password", routes![safe_update_password])
}