use std::{
    io::{stdout, Read, Write},
    sync::Arc,
};

#[macro_use]
extern crate rocket;

use rocket::{fs, response::content::RawHtml};
use rocket::{form, response::Responder};
use rocket::{http, shield::Shield};
use std::string::ToString;

struct CSPHeader {}
struct CSPHeaderVulnerable {}

// Local sanitizer for the relevant handler
impl<'r> Responder<'r, 'static> for CSPHeader {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::response::Response::build();
        response.raw_header("Content-Security-Policy", "default-src self"); // SANITIZER
        response.ok()
    }
}
// Local sanitizer for the relevant handler
impl<'r> Responder<'r, 'static> for CSPHeaderVulnerable {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::response::Response::build();
        response.raw_header("Content-Security-Policy", "default-src *"); // VULNERABLE
        response.ok()
    }
}

#[get("/safe")]
fn custom_header() -> CSPHeader {
    CSPHeader {}
}

#[get("/vulnerable")]
fn custom_header_2() -> CSPHeaderVulnerable {
    CSPHeaderVulnerable {}
}

#[get("/meta1")]
fn home_meta_safe() -> RawHtml<String> {
    rocket::response::content::RawHtml("<html><head><meta http-equiv='Content-Security-Policy' content='default-src https:' /></head><body><h1>hi</h1></body></html>".to_string())
}
#[get("/meta2")]
fn home_meta_vulnerable() -> RawHtml<String> {
    rocket::response::content::RawHtml("<html><head><meta http-equiv='Content-Security-Policy' content='default-src *' /></head><body><h1>hi</h1></body></html>".to_string())
}

#[get("/")]
fn home() -> String {
    "done".to_string()
}

#[launch]
fn rocket() -> _ {
    // taking a random variable from command line just so every flow will be possible
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        if args[1] == "0" {
            // Sanitized by custom fairing:
            rocket::build()
                .attach(rocket::fairing::AdHoc::on_response(
                    "test fairing",
                    |_, res| {
                        Box::pin(async move {
                            res.set_raw_header("Content-Security-Policy", "default-src self"); // SANITIZED - CUSTOM FAIRING SANITIZER
                        })
                    },
                ))
                .mount("/", routes![home]) // SAFE - this is sanitized by the custom fairing
        } else {
            rocket::build()
                .mount("/", routes![home]) // VULNERABLE - no custom fairing nor custom header
                .mount("/", routes![custom_header]) // THIS IS SANITIZED BY THE THE CSPHeader Struct
                .mount("/", routes![custom_header_2]) // VULNERABLE - Permissive CSPHeaderVulnerable Struct (custom header)
        }
    } else {
        rocket::build()
            .mount("/", routes![home]) // VULNERABLE - no custom fairing nor custom header
            .mount("/", routes![home_meta_safe]) // SAFE - meta tag sanitizer
            .mount("/", routes![home_meta_vulnerable]) // VULNERABLE - permissive meta tag
            .mount("/", routes![custom_header]) // SAFE - this is sanitized by the CSPHeader Struct (custom header)
            .mount("/", routes![custom_header_2]) // VULNERABLE - Permissive CSPHeaderVulnerable Struct (custom header)
    }
}
