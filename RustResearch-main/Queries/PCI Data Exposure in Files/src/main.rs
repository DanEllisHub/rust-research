#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};
use rocket::response::{content, status, Redirect};
use ring::digest::{digest, SHA256};
use std::str;
use std::fs;





pub fn get_pan(id: &String) -> String {
  //get pan from db
  return "123-45-6789".to_string();
}

//vulnerable code
#[get("/save_pan?<id>")]
pub fn save_pan(id: String) -> String { 
  let pan: String = get_pan(&id); //PII
  let path: String = "path\\to\\file".to_string();
  fs::write(path, pan); //sink is second param
  return "saved pan successfully!".to_string();
}


//safe code
#[get("/safe_save_pan?<id>")]
pub fn safe_save_pan(id: String) -> String { 
  let path: String = "path\\to\\file".to_string();
  let pan: String = get_pan(&id);
  let digested = digest(&SHA256, &pan.as_bytes()); //sanitizer
  let text: String = format!("pan for user {:?} : {:?}", &id, str::from_utf8(digested.as_ref()).unwrap());
  fs::write(path, text);
  return "saved pan successfully!".to_string();
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/save_pan", routes![save_pan])
        .mount("/safe_save_pan", routes![safe_save_pan])
}