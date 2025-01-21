#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};
use rocket::response::{content, status, Redirect};
use ring::digest::{digest, SHA256};
use std::str;
use std::fs;





pub fn get_ssn(id: &String) -> String {
  //get ssn from db
  return "123-45-6789".to_string();
}

//vulnerable code
#[get("/save_ssn?<id>")]
pub fn save_ssn(id: String) -> String { 
  let ssn: String = get_ssn(&id); //PII
  let path: String = "path\\to\\file".to_string();
  fs::write(path, ssn); //sink is second param
  return "saved ssn successfully!".to_string();
}


//safe code
#[get("/safe_save_ssn?<id>")]
pub fn safe_save_ssn(id: String) -> String { 
  let path: String = "path\\to\\file".to_string();
  let ssn: String = get_ssn(&id);
  let digested = digest(&SHA256, &ssn.as_bytes()); //sanitizer
  let text: String = format!("SSN for user {:?} : {:?}", &id, str::from_utf8(digested.as_ref()).unwrap());
  fs::write(path, text);
  return "saved ssn successfully!".to_string();
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/save_ssn", routes![save_ssn])
        .mount("/safe_save_ssn", routes![safe_save_ssn])
}