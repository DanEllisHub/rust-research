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

//asdasdasdasdsa
//vulnerable code
#[get("/?<token>&<key>")]
fn jwt(token: &str, key: &[u8]) -> String {
    let mut validation = Validation::new(Algorithm::HS256);
     //validation.validate_nbf= true; //this is a sanitizer for this query
    let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) { // sink
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"),
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"),
            _ => panic!("Some other errors"),
        },
    };
    format!("{:?}", token_data.header)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![jwt])
}