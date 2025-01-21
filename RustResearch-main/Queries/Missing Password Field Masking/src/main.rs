#[macro_use]
extern crate rocket;
 
use rocket::response::content::RawHtml;
use rocket::{fs::NamedFile};
use rocket::get;
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

//vulnerable code - "about" template displays a password without masking
#[get("/about?<password>")]
fn about(password: String) -> Template {
    Template::render("Tera/about", context! { attach: password })
}

//vulnerable code - response is sent with unmasked password
#[get("/?<name>&<pass>")]
fn rawhtml_hello(name: &str, pass: &str) -> RawHtml<String> {
    RawHtml(format!("<b>name: {}. password: {}</b>", name, pass)) // Sets content type to HTML
}

//safe code - type "password" masks the field
#[get("/info?<name>&<pass>")]
fn rawhtml_hello1(name: &str, pass: &str) -> RawHtml<String> {
    RawHtml(format!("<b>name: {}. <input name=\"pwd\" type=\"password\" value=\"{}\"></b>", name, pass)) // Sets content type to HTML
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![rawhtml_hello])
        .mount("/info", routes![rawhtml_hello1])
        .mount("/about", routes![about])
}