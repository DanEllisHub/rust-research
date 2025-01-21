#[macro_use]
extern crate rocket;
use rocket::form;
use rocket::futures::pin_mut;
use rocket::futures::StreamExt;
use rocket_db_pools::deadpool_postgres::Pool;
use rocket_db_pools::Connection;
use rocket_db_pools::Database;
use rocket_db_pools::deadpool_postgres;
use rocket_db_pools::Initializer;

static PATH: &str = "../../file-body.txt";

#[derive(Database)]
#[database("postgres")]
struct Logs(Pool);


use rocket_db_pools;
#[get("/<id>?<input>")]
async fn open_empty_password(mut db: &Logs, input: String, id: i64) -> String {
  let conf = rocket_db_pools::Config {
    url: "postgresql://postgres@localhost:5433".to_string(), // RESULT - empty password
    min_connections: None,
    max_connections: 1024,
    connect_timeout: 3,
    idle_timeout: None,
  };

  // ... rest of code
  "done".to_string()
}

#[get("/<id>?<input>")]
async fn open_hardcoded_password(mut db: &Logs, input: String, id: i64) -> String {
  let conf = rocket_db_pools::Config {
    url: "postgresql://postgres:root@localhost:5433".to_string(), // RESULT - hardcoded password
    min_connections: None,
    max_connections: 1024,
    connect_timeout: 3,
    idle_timeout: None,
  };

  // ... rest of code
  "done".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Logs::init())
        .mount("/", routes![open_empty_password])
        .mount("/", routes![open_hardcoded_password])
}
