#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use rocket::response;
use serde::{Deserialize, Serialize};

fn authenticate(user: &str, pass: &str) -> Bool {
    //fake authentication
    return true
}

//vulnerable code
#[get("/login?<user>&<pass>")]
fn login(user: &str, pass: &str) -> String {
    if user == "admin" && pass == "secretpassword" {
        let response: &str = "success!";
    }
    else {
        let response: &str = "failed!";
    }
    return response;
}

//safe code
#[get("/login?<user>&<pass>")]
fn login(user: &str, pass: &str) -> String {
    if (authenticate(user, pass)) {
        let response: &str = "success!";
    }
    else {
        let response: &str = "failed!";
    }
    return response;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![jwt])
        .mount("/safe", routes![jwt_safe])
}