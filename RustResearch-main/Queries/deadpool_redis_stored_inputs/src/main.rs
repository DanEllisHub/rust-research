#[macro_use]
extern crate rocket;
use rocket::form;
use rocket_db_pools;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::deadpool_redis::{
    redis::{cmd, Cmd, FromRedisValue},
    Config, Manager, Runtime,
};
use rocket_db_pools::Connection;
use std::env;
use std::fs;
use tokio_postgres::NoTls;

static PATH: &str = "../../file-body.txt";

#[derive(rocket_db_pools::Database)]
#[database("cache-LVTEAUDO")]
struct Logs(rocket_db_pools::deadpool_redis::Pool);

#[get("/get")]
async fn get(mut db: Connection<Logs>) -> String {
    let s: String = db // RESULT - `s` is stored input
        .get("key")
        .await
        .unwrap();
    // rest of code ...
    s
}


#[get("/get_pool_2")]
async fn get_pool_2(db: &Logs) -> String {
    let mut conn = db.0.get().await.unwrap();
    let s: String = cmd("GET") // Result - `s`is stored input
        .arg(&["key"])
        .query_async(&mut conn)
        .await
        .unwrap();
    // rest of code ...
    s
}

#[get("/get_pool")]
async fn get_pool(db: &Logs) -> String {
    let s: String = db // RESULT - `s` is stored input
        .0
        .get()
        .await
        .unwrap()
        .get("key")
        .await
        .unwrap();
    s
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
    use rocket_db_pools::Database;
    rocket::build()
        .attach(Logs::init())
        .mount("/", routes![get_pool])
        .mount("/", routes![get])
        .mount("/", routes![get_pool_2])
}
