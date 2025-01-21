use std::{
    io::{stdout, Read, Write},
    sync::Arc,
};

#[macro_use]
extern crate rocket;

use rocket::fs;
use rocket::{form, response::Responder};
use rocket::{http, shield::Shield};
use std::string::ToString;

struct CSPHeader {}

// Local sanitizer for the relevant handler
impl<'r> Responder<'r, 'static> for CSPHeader {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::response::Response::build();
        response.raw_header("Content-Security-Policy", "Any Value"); // SANITIZER
        response.ok()
    }
}

#[get("/safe")]
fn custom_header() -> CSPHeader {
    // THIS ENDPOINT IS SAFE NO MATTER IF FAIRING IS ATTACHED TO IT
    // WHEN A NO FAIRING IS ATTACHED, THE LOCAL SANITIZER (CUSTOM HEADER)
    // WILL TAKE PLACE AND SANITIZE THE RESPONSE
    CSPHeader {}
}

#[get("/")]
fn home() -> String {
    // safe/vulnerable depend on the shield
    "done".to_string()
}

#[launch]
fn rocket() -> _ {
    let shield_to_be_used: Shield;

    // taking a random variable from command line just so every flow will be possible
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        if args[1] == "0" {
            // Sanitized by custom fairing:
            rocket::build()
                .attach(Shield::new())
                .attach(rocket::fairing::AdHoc::on_response(
                    "test fairing",
                    |_, res| {
                        Box::pin(async move {
                            res.set_raw_header("Content-Security-Policy", "Any Value"); // SANITIZED - CUSTOM FAIRING SANITIZER
                        })
                    },
                ))
                .mount("/", routes![home]) // SAFE - this is sanitized by the custom fairing
        } else {
            rocket::build()
                .mount("/", routes![home]) // VULNERABLE - no custom fairing nor custom header
                .mount("/", routes![custom_header]) // THIS IS SANITIZED BY THE THE CSPHeader Struct
        }
    } else {
        rocket::build()
            .mount("/", routes![home]) // VULNERABLE - no custom fairing nor custom header
            .mount("/", routes![custom_header]) // SAFE - this is sanitized by the CSPHeader Struct (custom header)
    }
}
