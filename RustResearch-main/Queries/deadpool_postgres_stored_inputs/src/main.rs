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

#[get("/<id>")]
async fn read(mut db: &Logs, id: i64) -> String {
    let mut foo : String = "unintialized".to_string();
    let n = db.0
        .get()
        .await
        .unwrap()
        .query_raw("SELECT body FROM logs WHERE id = $1", &[&id])
        .await
        .unwrap();

        pin_mut!(n);
        while let Some(row) = n.next().await {
            foo = row.ok().unwrap().get(0); // RESULT - get() returns the value from the Row
        }

        
    format!("Hello {}", foo).to_string()
}

#[get("/r/<id>")]
async fn read_2(mut db: Connection<Logs>, id: i64) -> String {
    let n : String = db
        .query_one("SELECT body FROM logs WHERE id = $1", &[&id])
        .await
        .unwrap()
        .get(0); // RESULT - get() returns the value from the Row

    format!("Hello {}", n).to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Logs::init())
        .mount("/", routes![read])
        .mount("/", routes![read_2])
}
