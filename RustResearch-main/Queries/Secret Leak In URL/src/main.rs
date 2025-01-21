#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};
use reqwest;
use rocket::response::{content, status, Redirect};
use ring::digest::{digest, SHA256};
use std::str;

const URL: &str = "url.url/?key=";

pub fn get_key(id: &String) -> String {
  //get key from db
  return "123-45-6789".to_string();
}

//vulnerable code - flow 1
#[get("/send_key?<id>")]
pub fn send_key(id: String) -> String { 
  let key: String = get_key(&id);
  let body = reqwest::blocking::get(URL.to_owned() + &key);
  return body.unwrap().text().unwrap();
}

//vulnerable code - flow 2
#[get("/receive_key?<key>")]
pub fn receive_key(key: String) -> String { 
  save_key(&key, &"name".to_string());
  return "key received".to_string();
}

//vulnerable code - flow 3
#[get("/send_key2?<id>")]
pub fn send_key2(id: String) -> Redirect { 
  let key: String = get_key(&id);
  Redirect::to(URL.to_owned() + &key)
}

//safe code
#[get("/safe_send_key?<id>")]
pub fn safe_send_key(id: String) -> String { 
  let key: String = get_key(&id);
  let digested = digest(&SHA256, &key.as_bytes());
  let body = reqwest::blocking::get(URL.to_owned() + str::from_utf8(digested.as_ref()).unwrap());
  return body.unwrap().text().unwrap();
}

pub fn save_key(key: &String, name: &String) {
  //save id to db
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/send_key", routes![send_key])
        .mount("/receive_key", routes![receive_key])
        .mount("/send_key2", routes![send_key2])
        .mount("/safe_send_key", routes![safe_send_key])
}