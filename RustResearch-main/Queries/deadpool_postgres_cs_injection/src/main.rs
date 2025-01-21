#[macro_use]
extern crate rocket;
use rocket::form;
use rocket_db_pools::deadpool_postgres::Pool;
use rocket_db_pools::Connection;
use rocket_db_pools::Database;
use rocket_db_pools::deadpool_postgres;
use rocket_db_pools::Initializer;

static PATH: &str = "../../file-body.txt";

#[derive(Database)]
#[database("postgres")]
struct Logs(Pool);

#[get("/<id>?<input>")]
async fn read(mut db: &Logs, input: String, id: i64) -> String {
    let s = deadpool_postgres::Config{
        url: Some(input), // RESULT
        // ...
    };

    // ... rest of code
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Logs::init())
        .mount("/", routes![read])
        .mount("/", routes![read_2])
}
