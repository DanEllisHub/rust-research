#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    company: String,
    exp: u64,
    nbf: usize,
}
//vulnerable code
#[get("/?<aud>&<sub>&<company>&<key>")]
fn jwt(aud: &str, sub: &str,company: &str, key: &[u8]) -> String {
    let my_claims = Claims {
        aud: aud,
        sub: sub,
        company: company,
        exp: get_current_timestamp() + 100000 //excessive expiration time set
    };
    let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) { // sink
        Ok(t) => t,
        Err(_) => panic!(), 
    }
}
//safe code
#[get("/safe?<aud>&<sub>&<company>&<key>")]
fn jwt_safe(aud: &str, sub: &str,company: &str, key: &[u8]) -> String {
    let my_claims = Claims {
        aud: aud,
        sub: sub,
        company: company,
        exp: get_current_timestamp() + 900 //setting exp for 15 minutes
    };
    let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(_) => panic!(), 
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![jwt])
        .mount("/safe", routes![jwt_safe])
}