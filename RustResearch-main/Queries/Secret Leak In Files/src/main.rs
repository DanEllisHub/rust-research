#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};
use rocket::response::{content, status, Redirect};
use ring::digest::{digest, SHA256};
use std::str;
use std::fs;



//vulnerable code
#[get("/save_token?<token>")]
pub fn save_token(token: String) -> String { 
  let path: String = "path\\to\\file".to_string();
  fs::write(path, token); //sink is second param
  return "saved token successfully!".to_string();
}


//safe code
#[get("/safe_save_token?<token>")]
pub fn safe_save_token(token: String) -> String { 
  let path: String = "path\\to\\file".to_string();
  let digested = digest(&SHA256, &token.as_bytes()); //sanitizer
  let text: String = format!("digested token for user {:?} : {:?}", &id, str::from_utf8(digested.as_ref()).unwrap());
  fs::write(path, text);
  return "saved token successfully!".to_string();
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/save_token", routes![save_token])
        .mount("/safe_save_token", routes![safe_save_token])
}