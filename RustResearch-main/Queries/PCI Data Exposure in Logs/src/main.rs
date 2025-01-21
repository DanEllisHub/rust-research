#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};
use rocket::response::{content, status, Redirect};
use ring::digest::{digest, SHA256};
use log::{info, warn, error, debug, trace, log};
use std::str;

pub fn get_pan(id: &String) -> String {
  //get pan from db
  return "123-45-6789".to_string();
}

//vulnerable code
#[get("/log_pan?<id>")]
pub fn log_pan(id: String) -> String { 
  let pan: String = get_pan(&id); //PCI
  info!("pan for user {} : {}", &id, &pan); //result
  warn!("pan for user {} : {}", &id, &pan); //also result
  return "logged pan successfully!".to_string();
}


//safe code
#[get("/safe_log_pan?<id>")]
pub fn safe_log_pan(id: String) -> String { 
  let pan: String = get_pan(&id);
  let digested = digest(&SHA256, &pan.as_bytes()); //sanitizer
  info!("pan for user {:?} : {:?}", &id, str::from_utf8(digested.as_ref()).unwrap());
  return "logged pan successfully!".to_string();
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/log_pan", routes![log_pan])
        .mount("/safe_log_pan", routes![safe_log_pan])
}