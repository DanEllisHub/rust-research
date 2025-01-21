#[macro_use]
extern crate rocket;
use rocket::form;
use rocket_db_pools::deadpool_redis::{
    redis::{Cmd, cmd, FromRedisValue},
    Config, Manager, Runtime,
};
use std::env;
use std::fs;
use rocket_db_pools::Connection;
use tokio_postgres::NoTls;

static PATH: &str = "../../file-body.txt";

#[get("/open_redis?<input>")]
async fn open_redis(input: String) -> String {    
    let mut k = Config::from_url(input.as_str()); // RESULT #1 - Connection String Injection
    k.url = Some(input); // RESULT #2 - Connection String Injection

    // rest of code ...
    "done".to_string()
}


fn whitelist(input: &str) -> String {
    if input == "something" {
        return "something".to_string();
    } else {
        return "default".to_string();
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![open_redis])
}
