#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};
use reqwest;
use rocket::response::{content, status, Redirect};
use ring::digest::{digest, SHA256};
use std::str;

const URL: &str = "url.url/?ssn=";

pub fn get_ssn(id: &String) -> String {
  //get ssn from db
  return "123-45-6789".to_string();
}

//vulnerable code - flow 1
#[get("/send_ssn?<id>")]
pub fn send_ssn(id: String) -> String { 
  let ssn: String = get_ssn(&id);
  let body = reqwest::blocking::get(URL.to_owned() + &ssn);
  return body.unwrap().text().unwrap();
}

//vulnerable code - flow 2
#[get("/receive_ssn?<ssn>")]
pub fn receive_ssn(ssn: String) -> String { 
  save_ssn(&ssn, &"name".to_string());
  return "SSN received".to_string();
}

//vulnerable code - flow 3
#[get("/send_ssn2?<id>")]
pub fn send_ssn2(id: String) -> Redirect { 
  let ssn: String = get_ssn(&id);
  Redirect::to(URL.to_owned() + &ssn)
}

//safe code
#[get("/safe_send_ssn?<id>")]
pub fn safe_send_ssn(id: String) -> String { 
  let ssn: String = get_ssn(&id);
  let digested = digest(&SHA256, &ssn.as_bytes());
  let body = reqwest::blocking::get(URL.to_owned() + str::from_utf8(digested.as_ref()).unwrap());
  return body.unwrap().text().unwrap();
}

pub fn save_ssn(ssn: &String, name: &String) {
  //save id to db
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/send_ssn", routes![send_ssn])
        .mount("/receive_ssn", routes![receive_ssn])
        .mount("/send_ssn2", routes![send_ssn2])
        .mount("/safe_send_ssn", routes![safe_send_ssn])
}