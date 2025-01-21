mod missing_framing_policy;

use std::{
    io::{stdout, Read, Write},
    sync::Arc,
};

#[macro_use]
extern crate rocket;

use rocket::{form, response::Responder};
use rocket::fs;
use rocket::{http, shield::Shield};
use std::string::ToString;


struct FramingPolicyHeader{

}
struct FramingPolicyHeader2{

}

// Local sanitizer for the relevant handler
impl<'r> Responder<'r, 'static> for FramingPolicyHeader {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::response::Response::build();
        response.raw_header("X-Frame-Options", "SAMEORIGIN"); // SANITIZER
        response.ok()
    }
}

// Local sanitizer for the relevant handler
impl<'r> Responder<'r, 'static> for FramingPolicyHeader2 {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::response::Response::build();
        response.raw_header("Content-Security-Policy", "frame-ancestors 'none'"); // SANITIZER
        response.ok()
    }
}


#[get("/safe")]
fn login() -> FramingPolicyHeader {
    // THIS ENDPOINT IS SAFE NO MATTER WHICH SHIELD IS ATTACHED TO IT
    // WHEN A VULNERABLE SHIELD IS ATTACHED, THE LOCAL SANITIZER (CUSTOM HEADER)
    // WILL TAKE PLACE AND SANITIZE THE RESPONSE
    FramingPolicyHeader{}
}

#[get("/login")]
fn safeEndpoint() -> FramingPolicyHeader2 {
    // THIS ENDPOINT IS SAFE NO MATTER WHICH SHIELD IS ATTACHED TO IT
    // WHEN A VULNERABLE SHIELD IS ATTACHED, THE LOCAL SANITIZER (CUSTOM HEADER)
    // WILL TAKE PLACE AND SANITIZE THE RESPONSE
    FramingPolicyHeader2{}
}


#[get("/sign-in")]
fn sanitizedWithGlobalSanitizer() -> String {
    "done".to_string()
}


#[get("/")]
fn home() -> String {
    // safe/vulnerable depend on the shield
    "done".to_string()
}

#[launch]
fn rocket() -> _ {
    let shield_to_be_used : Shield;
    
    // taking a random variable from command line just so every flow will be possible
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        if args[1] == "0" {
            // vulnerable to frameable login page
            shield_to_be_used = missing_framing_policy::missing_framing_policy::get_vulnerable_shield_0();
        }
        else if args[1] == "1" {
            // vulnerable to frameable login page
            shield_to_be_used = missing_framing_policy::missing_framing_policy::get_vulnerable_shield_1();
        }
        else if args[1] == "2" {
            // SAFE to frameable login page
            shield_to_be_used = missing_framing_policy::missing_framing_policy::get_safe_shield_2();
        }
        else if args[1] == "3" {
            // SAFE to frameable login page
            shield_to_be_used = missing_framing_policy::missing_framing_policy::get_safe_shield_3();
        }
        else {
            // SAFE to frameable login page
            shield_to_be_used = missing_framing_policy::missing_framing_policy::get_safe_shield_4();
        }

        rocket::build()
            .attach(shield_to_be_used) 
            .mount("/", routes![home]) // not relevant
            .mount("/", routes![safeEndpoint]) // safe
            .mount("/", routes![login]) // safe
            .mount("/", routes![sanitizedWithGlobalSanitizer]) // THIS IS SAFE/VULNERABLE DEPENDENT ON THE SHIELD ABOVE
    }
    else if args.len() == 2{
        rocket::build() // SAFE - shield doesn't need to be attached explicitly
            .mount("/", routes![home])
    }
    else {
        rocket::build()
            .attach(Shield::new())
            .mount("/", routes![home]) // not relevant
            .mount("/", routes![safeEndpoint]) // safe
            .mount("/", routes![login]) // safe
            .mount("/", routes![sanitizedWithGlobalSanitizer]) // vulnerable
    }

}
