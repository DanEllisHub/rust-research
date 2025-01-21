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
#[get("/?<aud>&<sub>&<company>")]
fn jwt(aud: &str, sub: &str,company: &str) -> String {
    let key = b"secret"; //hardcoded secret 
    let my_claims = Claims {
        aud: aud,
        sub: sub,
        company: company,
    };
    let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(_) => panic!(), 
    }
}
//safe code
#[get("/safe?<aud>&<sub>&<company>")]
fn jwt_safe(aud: &str, sub: &str,company: &str) -> String { 
    let key: &[u8] = match std::env::var_os("jwt_secret") //storing secret as env variable
    let my_claims = Claims {
        aud: aud,
        sub: sub,
        company: company,
        exp: get_current_timestamp() + 900
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