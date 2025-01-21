use std::{
    io::{stdout, Read, Write},
    sync::Arc,
};

#[macro_use]
extern crate rocket;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{form, response::Responder, time::Duration};
use rocket::fs;
use rocket::{http, shield::Shield};
use std::string::ToString;
use rocket::http::Header;

pub struct UnsafeCorsFairing;

#[rocket::async_trait]
impl Fairing for UnsafeCorsFairing {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
    }
}


pub struct SafeCorsFairing;

#[rocket::async_trait]
impl Fairing for SafeCorsFairing {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "https://example.com"));
    }
}



struct UnsafeResponder{

}

// local sanitizer - custom responder
impl<'r> Responder<'r, 'static> for UnsafeResponder {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::response::Response::build();
        response.raw_header("Access-Control-Allow-Origin", "*"); // SANITIZER
        response.ok()
    }
}




#[get("/unsafe")]
fn custom_header() -> UnsafeResponder {
    UnsafeResponder{}
}


struct SafeResponder{

}

// Local sanitizer for the relevant handler
impl<'r> Responder<'r, 'static> for SafeResponder {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::response::Response::build();
        response.raw_header("Access-Control-Allow-Origin", "https://example.com"); // NOT A SANITIZER
        response.ok()
    }
}

#[get("/safe")]
fn custom_header_2() -> SafeResponder {
    SafeResponder{}
}


#[get("/")]
fn home() -> String {
    // safe/vulnerable depend on the shield
    "done".to_string()
}

#[launch]
fn rocket() -> _ {
    // taking a random variable from command line just so every flow will be possible
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 3{
        rocket::build()
            .attach(SafeCorsFairing) // SANITIZER
            .mount("/", routes![home]) // SAFE
            .mount("/", routes![custom_header]) // SAFE - Desanitized locally, but that means it's intentionally
            .mount("/", routes![custom_header_2]) // SAFE
    }
    else if args.len() == 4{
        rocket::build()
            .attach(UnsafeCorsFairing) // RESULT
            .mount("/", routes![home]) // VULNERABLE
            .mount("/", routes![custom_header]) // VULNERABLE
            .mount("/", routes![custom_header_2]) // SAFE - LOCAL SANITIZER
    }
    else if args.len() == 5{
        // VULNERABLE
        rocket::build()
            .attach(rocket::fairing::AdHoc::on_response(
                "test fairing",
                |_, res| {
                    // VULNERABLE
                    Box::pin(async move {
                        res.set_raw_header("Access-Control-Allow-Origin", "*"); // RESULT
                    })
                },
            ))
            .mount("/", routes![home]) // RESULT - VULNERABLE
            .mount("/", routes![custom_header]) // RESULT - VULNERABLE
            .mount("/", routes![custom_header_2]) // SAFE
    }
    else if(args.len() == 6){
        // SAFE
        rocket::build()
            .attach(rocket::fairing::AdHoc::on_response(
                "test fairing",
                |_, res| {
                    // SAFE
                    Box::pin(async move {
                        res.set_raw_header("Access-Control-Allow-Origin", "https://example.com"); // SANITIZER
                    })
                },
            ))
            .mount("/", routes![home]) // SAFE
            .mount("/", routes![custom_header]) // SAFE
            .mount("/", routes![custom_header_2]) // SAFE
    }
    else {
        rocket::build()
            .mount("/", routes![home]) // SAFE
            .mount("/", routes![custom_header]) // VULNERABLE
            .mount("/", routes![custom_header_2]) // SAFE
    }
}