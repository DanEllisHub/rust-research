use std::{
    io::{stdout, Read, Write},
    sync::Arc,
};

#[macro_use]
extern crate rocket;
use jsonwebtoken::jwk::AlgorithmParameters;
use jsonwebtoken::{decode, decode_header, jwk, Algorithm, DecodingKey, Validation};
use rocket::fs;
use rocket::{form, response::Responder};
use rocket::{http, shield::Shield};
use std::string::ToString;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    aud: String,         // Optional. Audience
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
    uid: String,         // Custom field
    role: String,        // Custom field
}

struct RefererGuard {}
struct AuthGuard {}

// VULNERABLE GUARD - Results will flow from here
#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for RefererGuard {
    type Error = ();
    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let referer = request.headers().get_one("Referer");
        match referer {
            Some(referer) => {
                if referer.contains("someValueForRefererHeader.com") { // STEP #1 - CONDITION TO CHECK REFERER
                    println!("Referer is valid");
                    rocket::request::Outcome::Success(RefererGuard {}) // STEP #2.1 - SUCCESS
                } else {
                    println!("Referer is invalid");
                    rocket::request::Outcome::Failure((rocket::http::Status::new(401), ())) // STEP #2.2 - NOT AUTHORIZED (401)
                }
            }
            None => rocket::request::Outcome::Failure((rocket::http::Status::new(403), ())), // STEP #2.3 - FORBIDDEN (403)
        }
    }
}


// THIS IS A VALID SANTIZER
#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for AuthGuard {
    type Error = ();
    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let token_cookie = request.cookies().get("jwt").expect("no cookie was found").value();
        let token: jsonwebtoken::TokenData<Claims> = decode::<Claims>(&token_cookie, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()).expect("something went wrong");
        let is_authenticated = token.claims.role == "user" || token.claims.role == "admin";
        if is_authenticated {
            rocket::request::Outcome::Success(AuthGuard {})
        } else {
            rocket::request::Outcome::Failure((rocket::http::Status::new(401), ()))
        }
    }
}

#[get("/referer")]
fn referer_guard(_guard: RefererGuard) -> String {
    "vulnerable".to_string() // VULNERABLE
}

#[get("/guarded")]
fn auth_guard(_guard: RefererGuard, _auth: AuthGuard) -> String {
    "safe".to_string() // SAFE - sanitized by `AuthGuard`
}

#[get("/guarded_2")]
fn auth_guard_2(_auth: AuthGuard) -> String {
    "safe".to_string() // SAFE - sanitized by `AuthGuard`
}

#[get("/")]
fn home() -> String {
    "safe".to_string() // SAFE
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home]) // SAFE
        .mount("/", routes![referer_guard]) // VULNERABLE
        .mount("/", routes![auth_guard]) // SAFE
        .mount("/", routes![auth_guard_2]) // SAFE
}
