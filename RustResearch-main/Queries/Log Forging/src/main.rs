mod log_forging_example;
use std::{io::{stdout, Read, Write}, sync::Arc};

#[macro_use] extern crate rocket;

use rocket::http;
use rocket::form;
use rocket::fs;
use std::string::ToString;


#[get("/<name>")]
fn log(name: &str) -> String {
    log_forging_example::log_forging_example::forge(name);
    log_forging_example::log_forging_example::safe(name);
    log_forging_example::log_forging_example::safe_2(name);
    "done".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/log", routes![log])
}
