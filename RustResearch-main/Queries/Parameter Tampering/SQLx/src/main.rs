#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;

#[cfg(test)] mod tests;

mod sqlx;
mod safe;


use rocket::response::Redirect;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/sqlx", sqlx::list()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(sqlx::stage())
        .attach(safe::stage())
}
