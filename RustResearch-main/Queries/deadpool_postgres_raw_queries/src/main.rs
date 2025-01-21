#[macro_use]
extern crate rocket;
use rocket::form;
use rocket_db_pools::deadpool_postgres::Pool;
use rocket_db_pools::Connection;
use rocket_db_pools::Database;

static PATH: &str = "../../file-body.txt";

#[derive(Database)]
#[database("postgres")]
struct Logs(Pool);

#[get("/<id>?<qry>")]
async fn read(mut db: &Logs, qry: String, id: i64) -> String {

    // `qry` is an input
    let query = format!("{} body FROM logs WHERE id = $1", qry); // input embedded in query

    let n : String = db.0
        .get()
        .await
        .unwrap()
        .query_one(query.as_str(), &[&id]) // first parameter is a sink
        .await
        .unwrap()
        .get(0);

    format!("Hello {}", n).to_string()
}

#[get("/<id>?<qry2>")]
async fn read_2(mut db: Connection<Logs>, qry2: String, id: i64) -> String {

    // `qry` is an input
    let query = format!("{} body FROM logs WHERE id = $1", qry2); // input embedded in query

    let n : String = db
        .query_one(query.as_str(), &[&id]) // first parameter is a sink
        .await
        .unwrap()
        .get(0);

    format!("Hello {}", n).to_string()
}

#[get("/db?<input>")]
async fn db(input: String) -> String {
    input
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
        .attach(Logs::init())
        .mount("/", routes![db])
        .mount("/", routes![read])
        .mount("/", routes![read_2])
}
