mod Misconfigured_X_Content_Type_Options;

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


struct NoSniff1{

}

// local sanitizer - custom responder
impl<'r> Responder<'r, 'static> for NoSniff1 {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::response::Response::build();
        response.raw_header("X-Content-Type-Options", "nosniff"); // SANITIZER
        response.ok()
    }
}




#[get("/safe")]
fn custom_header() -> NoSniff1 {
    NoSniff1{}
}


struct NoSniff2{

}

// Local sanitizer for the relevant handler
impl<'r> Responder<'r, 'static> for NoSniff2 {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::response::Response::build();
        response.raw_header("X-Content-Type-Options", "abcdef"); // NOT A SANITIZER
        response.ok()
    }
}

#[get("/safe2")]
fn custom_header_2() -> NoSniff2 {
    NoSniff2{}
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
            // vulnerable to misconfigured X Content Type Options
            shield_to_be_used = Misconfigured_X_Content_Type_Options::Misconfigured_X_Content_Type_Options::get_vulnerable_shield_0();
        }
        else if args[1] == "1" {
            // vulnerable to misconfigured X Content Type Options
            shield_to_be_used = Misconfigured_X_Content_Type_Options::Misconfigured_X_Content_Type_Options::get_vulnerable_shield_1();
        }
        else if args[1] == "2" {
            // SAFE to misconfigured X Content Type Options
            shield_to_be_used = Misconfigured_X_Content_Type_Options::Misconfigured_X_Content_Type_Options::get_safe_shield_2();
        }
        else if args[1] == "3" {
            // SAFE to misconfigured X Content Type Options
            shield_to_be_used = Misconfigured_X_Content_Type_Options::Misconfigured_X_Content_Type_Options::get_safe_shield_3();
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
                        res.set_raw_header("X-Content-Type-Options", "nosniff");
                    })
                },
            ))
            .mount("/", routes![home]) // SAFE
            .mount("/", routes![custom_header]) // VULNERABLE
            .mount("/", routes![custom_header_2]) // SAFE
    }
    else if args.len() > 4{
        // VULNERABLE
        rocket::build()
            .attach(Shield::new()) // Removing default shield
            .attach(rocket::fairing::AdHoc::on_response( // Not a sanitzier
                "test fairing",
                |_, res| {
                    Box::pin(async move {
                        res.set_raw_header("X-Content-Type-Options", "abcdef");
                    })
                },
            ))
            .mount("/", routes![home]) // VULNERABLE
            .mount("/", routes![custom_header]) // VULNERABLE
            .mount("/", routes![custom_header_2]) // SAFE
    }
    else {
        print!("\n\n\n\nIN THE LAST ROCKET\n\n\n\n");
        rocket::build()
            .attach(Shield::default()
                .disable::<rocket::shield::NoSniff>() // DESANITIZER
            )
            .mount("/", routes![home]) // VULNERABLE
            .mount("/", routes![custom_header]) // SAFE
            .mount("/", routes![custom_header_2]) // VULNERABLE
    }
}
