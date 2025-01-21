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

#[get("/open_redis_empty_pass")]
async fn open_redis_empty_password() -> String {
    let url = "redis://user@localhost:6379";
    let mut k = Config::from_url(url); // RESULT #1 - Empty password
    k.url = Some(url.to_string()); // RESULT #2 - Empty password

    // rest of code ...
    "done".to_string()
}

#[get("/open_redis_hardcoded_pass")]
async fn open_redis_hardcoded_password() -> String {
    let url = "redis://user:password@localhost:6379";
    let mut k = Config::from_url(url); // RESULT #1 - Hardcoded password
    k.url = Some(url.to_string()); // RESULT #2 - Hardcoded password

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
    rocket::build()
        .mount("/", routes![open_redis_empty_password])
        .mount("/", routes![open_redis_hardcoded_password])
}
