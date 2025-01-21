mod misconfigured_hsts;

use std::{
    io::{stdout, Read, Write},
    sync::Arc,
};

#[macro_use]
extern crate rocket;

use rocket::{form, response::Responder, time::Duration};
use rocket::fs;
use rocket::{http, shield::Shield};
use std::string::ToString;


struct HSTS1{

}

// Local sanitizer for the relevant handler
impl<'r> Responder<'r, 'static> for HSTS1 {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::response::Response::build();
        response.raw_header("Strict-Transport-Security", "max-age=31536000"); // SANITIZER
        response.ok()
    }
}




#[get("/vulnerable")]
fn custom_header() -> HSTS1 {
    // THIS ENDPOINT IS SAFE NO MATTER WHICH SHIELD IS ATTACHED TO IT
    // WHEN A VULNERABLE SHIELD IS ATTACHED, THE LOCAL SANITIZER (CUSTOM HEADER)
    // WILL TAKE PLACE AND SANITIZE THE RESPONSE
    HSTS1{}
}


struct HSTS2{

}

// Local sanitizer for the relevant handler
impl<'r> Responder<'r, 'static> for HSTS2 {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::response::Response::build();
        response.raw_header("Strict-Transport-Security", "max-age=31536000;includeSubDomains; preload"); // SANITIZER
        response.ok()
    }
}

#[get("/safe2")]
fn custom_header_2() -> HSTS2 {
    // THIS ENDPOINT IS SAFE NO MATTER WHICH SHIELD IS ATTACHED TO IT
    // WHEN A VULNERABLE SHIELD IS ATTACHED, THE LOCAL SANITIZER (CUSTOM HEADER)
    // WILL TAKE PLACE AND SANITIZE THE RESPONSE
    HSTS2{}
}


#[get("/")]
fn home() -> String {
    // safe/vulnerable depend on the shield
    "done".to_string()
}

#[launch]
fn rocket() -> _ {
    let mut shield_to_be_used : Shield = Shield::new();
    
    // taking a random variable from command line just so every flow will be possible
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 3{
        if args[1] == "0" {
            // vulnerable to misconfigured HSTS
            shield_to_be_used = misconfigured_hsts::misconfigured_hsts::get_vulnerable_shield_0();
        }
        else if args[1] == "1" {
            // vulnerable to misconfigured HSTS
            shield_to_be_used = misconfigured_hsts::misconfigured_hsts::get_vulnerable_shield_1();
        }
        else if args[1] == "11" {
            // vulnerable to misconfigured HSTS
            shield_to_be_used = misconfigured_hsts::misconfigured_hsts::get_vulnerable_shield_2();
        }
        else if args[1] == "2" {
            // SAFE to misconfigured HSTS
            shield_to_be_used = misconfigured_hsts::misconfigured_hsts::get_safe_shield_2();
        }
        else if args[1] == "3" {
            // SAFE to misconfigured HSTS
            shield_to_be_used = misconfigured_hsts::misconfigured_hsts::get_safe_shield_3();
        }        

        rocket::build()
            .attach(shield_to_be_used) 
            .mount("/", routes![home]) // THIS IS SAFE/VULNERABLE DEPENDENT ON THE COMMAND LINE ARGUMENT
            .mount("/", routes![custom_header]) // VULNERABLE
            .mount("/", routes![custom_header_2]) // SAFE
    }
    else if args.len() == 4{
        
        // everything here is safe

        rocket::build()
            .attach(Shield::new()) // Removing default shield
            .attach(rocket::fairing::AdHoc::on_response( // SANITIZER
                "test fairing",
                |_, res| {
                    Box::pin(async move {
                        res.set_raw_header("Strict-Transport-Security", "max-age=31536000;includeSubDomains; preload");
                    })
                },
            ))
            .mount("/", routes![home]) // SAFE
            .mount("/", routes![custom_header]) // VULNERABLE
            .mount("/", routes![custom_header_2]) // SAFE
    }
    else if args.len() > 4{
        
        rocket::build()
            .attach(Shield::new()) // Removing default shield
            .attach(rocket::fairing::AdHoc::on_response( // SANITIZER
                "test fairing",
                |_, res| {
                    Box::pin(async move {
                        res.set_raw_header("Strict-Transport-Security", "max-age=31536000;");
                    })
                },
            ))
            .mount("/", routes![home]) // Result
            .mount("/", routes![custom_header]) // VULNERABLE
            .mount("/", routes![custom_header_2]) // SAFE
    }
    else {
        // NO SHIELD IS ATTACHED EXPLICITLY BUT THE DEFAULT SHIELD IS ATTACHED AUTOMATICALLY
        // HENCE:
        // SAFE TO - misconfigured Framing Policy
        // SAFE TO - Misconfigured X-Content-Type-Options
        // SAFE TO - Misconfigured HSTS Header (if `tls` feature is enabled in cargo.toml)
        // SAFE TO - misconfigured HSTS Header (if `tls` feature is enabled in cargo.toml)
        // VULNERABLE TO - ANY OTHER HEADER-RELATED QUERY
        // reference : https://checkmarx.atlassian.net/wiki/spaces/AP/pages/7248445559/Rust+-+Rocket+s+Shield#Shield%3A%3Adefault()
        print!("\n\n\n\nIN THE LAST ROCKET\n\n\n\n");
        rocket::build()
            .attach(Shield::default()
                .disable::<rocket::shield::NoSniff>()
                .enable(rocket::shield::Hsts::IncludeSubDomains(rocket::time::Duration::days(365)))
            )
            .mount("/", routes![home]) // SAFE
            .mount("/", routes![custom_header]) // VULNERABLE
            .mount("/", routes![custom_header_2]) // SAFE
    }
}
