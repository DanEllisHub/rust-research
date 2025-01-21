#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};
use rocket::response::{content, status, Redirect};
use ring::digest::{digest, SHA256};
use log::{info, warn, error, debug, trace, log};
use std::str;

pub fn get_ssn(id: &String) -> String {
  //get ssn from db
  return "123-45-6789".to_string();
}

//vulnerable code
#[get("/log_ssn?<id>")]
pub fn log_ssn(id: String) -> String { 
  let ssn: String = get_ssn(&id); //PII
  info!("SSN for user {} : {}", &id, &ssn); //result
  warn!("SSN for user {} : {}", &id, &ssn); //also result
  return "logged ssn successfully!".to_string();
}


//safe code
#[get("/safe_log_ssn?<id>")]
pub fn safe_log_ssn(id: String) -> String { 
  let ssn: String = get_ssn(&id);
  let digested = digest(&SHA256, &ssn.as_bytes()); //sanitizer
  info!("SSN for user {:?} : {:?}", &id, str::from_utf8(digested.as_ref()).unwrap());
  return "logged ssn successfully!".to_string();
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/log_ssn", routes![log_ssn])
        .mount("/safe_log_ssn", routes![safe_log_ssn])
}