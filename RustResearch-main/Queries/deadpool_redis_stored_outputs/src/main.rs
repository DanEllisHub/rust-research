#[macro_use]
extern crate rocket;
use rocket::form;
use rocket_db_pools;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::deadpool_redis::{

    redis::{Cmd, cmd, FromRedisValue},
    Config, Manager, Runtime,
};
use std::env;
use std::fs;
use rocket_db_pools::Connection;
use tokio_postgres::NoTls;

static PATH: &str = "../../file-body.txt";


#[derive(rocket_db_pools::Database)]
#[database("cache-LVTEAUDO")]
struct Logs(rocket_db_pools::deadpool_redis::Pool);

#[get("/set")]
async fn set(mut db: Connection<Logs>) -> String {
    let s : String = db
                      .set("key", "value") // RESULT - "value" is stored output
                      .await.unwrap();
    // rest of code ...
    "done".to_string()
}

#[get("/append")]
async fn append(mut db: Connection<Logs>) -> String {
    let s : String = db
                    .append("key_a", "value") // RESULT - "value" is stored output
                    .await.unwrap();

    // rest of code ...
    "done".to_string()
}

#[get("/set_pool")]
async fn set_pool(db: &Logs) -> String {
    let s : String = db.0.get().await.unwrap()
                        .set("key", "value") // RESULT - "value" is stored output
                        .await.unwrap();
    // rest of code ...
    "done".to_string()
}

#[get("/set_pool_2")]
async fn set_pool_2(db: &Logs) -> String {
    let mut conn = db.0.get().await.unwrap();
    cmd("SET")
        .arg(&["new", "42"]) // RESULT - stored output
        .query_async::<_, ()>(&mut conn)
        .await
        .unwrap();
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
    use rocket_db_pools::Database;
    rocket::build()
        .attach(Logs::init())
        .mount("/", routes![append])
        .mount("/", routes![set])
        .mount("/", routes![set_pool])
        .mount("/", routes![set_pool_2])
}
