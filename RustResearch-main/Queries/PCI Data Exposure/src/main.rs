#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Valpanation, get_current_timestamp};
use serde::{Deserialize, Serialize};

pub fn get_pan(id: &str) -> String{
  return "123-456-789".to_string()
}

//vulnerable code
#[get("/pan?<id>")]
fn pan(id: String) -> String { 
    let pan: String = get_pan(&id);
    return pan
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/pan", routes![pan])
}