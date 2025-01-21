#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};
use reqwest;
use rocket::response::{content, status, Redirect};
use ring::digest::{digest, SHA256};
use std::str;

const URL: &str = "url.url/?pan=";

pub fn get_pan(id: &String) -> String {
  //get pan from db
  return "123-45-6789".to_string();
}

//vulnerable code - flow 1
#[get("/send_pan?<id>")]
pub fn send_pan(id: String) -> String { 
  let pan: String = get_pan(&id);
  let body = reqwest::blocking::get(URL.to_owned() + &pan);
  return body.unwrap().text().unwrap();
}

//vulnerable code - flow 2
#[get("/receive_pan?<pan>")]
pub fn receive_pan(pan: String) -> String { 
  save_pan(&pan, &"name".to_string());
  return "pan received".to_string();
}

//vulnerable code - flow 3
#[get("/send_pan2?<id>")]
pub fn send_pan2(id: String) -> Redirect { 
  let pan: String = get_pan(&id);
  Redirect::to(URL.to_owned() + &pan)
}

//safe code
#[get("/safe_send_pan?<id>")]
pub fn safe_send_pan(id: String) -> String { 
  let pan: String = get_pan(&id);
  let digested = digest(&SHA256, &pan.as_bytes());
  let body = reqwest::blocking::get(URL.to_owned() + str::from_utf8(digested.as_ref()).unwrap());
  return body.unwrap().text().unwrap();
}

pub fn save_pan(pan: &String, name: &String) {
  //save id to db
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/send_pan", routes![send_pan])
        .mount("/receive_pan", routes![receive_pan])
        .mount("/send_pan2", routes![send_pan2])
        .mount("/safe_send_pan", routes![safe_send_pan])
}