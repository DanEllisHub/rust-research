#[macro_use] extern crate rocket;

mod session;

use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"<a href="session">Use Sessions</a>."#)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/session", session::routes())
}