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

#[get("/")]
async fn write(mut db: &Logs) -> String {
    let params: Vec<String> = vec![
    "THIS IS AN OUTPUT".into(), // flow starts here, but result should be flagged below (the parameter of `query_raw`)
    ];

    let n = db.0
        .get()
        .await
        .unwrap()
        .query_raw("INSERT into logs (id, body) VALUES (2, $1)", params) // RESULT - `params` of `INSERT` is an output (writing to DB)
        .await
        .unwrap()
        .rows_affected()
        .expect("no value at all");
    format!("Hello {}", n).to_string()
}

#[get("/w")]
async fn write_2(mut db: &Logs) -> String {
    let n = db.0
        .get()
        .await
        .unwrap()
        .query("INSERT into logs (id, body) VALUES (3, $1)", &[&"THIS IS AN OUTPUT"]) // RESULT - `params` of `INSERT` is an output (writing to DB)
        .await
        .unwrap()
        .len() as i64;

    format!("Hello {}", n).to_string()
}

#[get("/w3")]
async fn write_3(mut db: &Logs) -> String {
    let n = db.0
        .get()
        .await
        .unwrap()
        .query("INSERT into logs (id, body) VALUES (3, $1)", &[&"THIS IS AN OUTPUT"]) // RESULT - `params` of `INSERT` is an output (writing to DB)
        .await
        .unwrap()
        .len() as i64;

    format!("Hello {}", n).to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Logs::init())
        .mount("/", routes![write])
        .mount("/", routes![write_2])
        .mount("/", routes![write_3])
}
