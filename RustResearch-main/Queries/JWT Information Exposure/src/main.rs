#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};
use ring::{digest::{self, SHA256_OUTPUT_LEN}, hmac, rand, signature, hkdf::{Salt, HKDF_SHA256, Prk}};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    company: String,
    exp: u64,
    ssn: String, //sensitive field
}
//vulnerable code
#[get("/?<aud>&<sub>&<company>")]
fn jwt(aud: &str, sub: &str,company: &str, ssn: &str) -> String {
    let key: &[u8] = match std::env::var_os("jwt_secret")
    let my_claims = Claims {
        aud: aud,
        sub: sub,
        company: company,
        exp: get_current_timestamp() + 900,
        ssn: ssn //sensitive field in JWT claim
    };
    let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) { //result - encoding JWT with sensitive info
        Ok(t) => t,
        Err(_) => panic!(), 
    }
}
//safe code
#[get("/safe?<aud>&<sub>&<company>")]
fn jwt_safe(aud: &str, sub: &str,company: &str,ssn: &str) -> String { 
    let key: &[u8] = match std::env::var_os("jwt_secret")
    let digested_ssn = digest::digest(&digest::SHA256, ssn); //hashing SSN before including in JWT claim
    let my_claims = Claims {
        aud: aud,
        sub: sub,
        company: company,
        exp: get_current_timestamp() + 900,
        ssn: digested_ssn
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