#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;
use rocket::time::Date;
use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};
use rocket::fs::{FileServer, TempFile, relative};
use rocket::{State, Shutdown, Response};
use std::string::ToString;
use rocket::response::stream::{EventStream, Event};
use rocket::serde::{Serialize, Deserialize,  json::Json};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::time::Duration;
use rocket::tokio::select;
use std::io::Cursor;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::fmt::Debug;
use rocket::{Rocket, Request, Data, Build,};
use rocket::fairing::{self, AdHoc, Fairing, Info, Kind};
use rocket::http::{Header,CookieJar,Method, Cookie};
use rocket::http::uri::Host;
use rocket::request::FromRequest;
use rocket::route::Route;
use rocket_dyn_templates::{Template, handlebars, tera::Tera, context};
use rocket::response::content::{RawHtml, RawXml};
use rocket::http::{Status, ContentType};
use rocket::http::uri::{Uri, Absolute, Origin};
use rocket::http::RawStr;
use std::fs::read_to_string;

#[get("/?<name>")]
fn plain_hello(name: &str) -> String {
    format!("Welcome {}", name) // FP
}

#[get("/?<name>")]
fn contenttype_hello(name: &str) -> (ContentType, String) {
    (ContentType::HTML, format!("Welcome {}", name)) // TP
}

#[get("/")]
fn print_file_hello() -> (ContentType, String) {
    let filename = "/test.txt";
    let result = read_to_string(filename).unwrap();
    
    (ContentType::HTML, format!("File: {}", result)) // TP
}

#[get("/?<name>")]
fn rawhtml_hello(name: &str) -> RawHtml<String> {
    RawHtml(format!("Welcome {}", name)) // TP
}

// Xmls also vulnerable to XSS, given the right injection point
#[get("/?<name>")]
fn contenttype_xml_hello(name: &str) -> (ContentType, String) {
    (ContentType::XML, String::from(name)) // TP
}

#[get("/?<name>")]
fn rawxml_hello(name: &str) -> RawXml<String> {
    RawXml(String::from(name)) // TP
}

#[get("/?<name>")]
fn hello(name: &str) -> String {
    //let uri: Origin = Origin::parse(name).expect("valid URI");
    let raw_str: &RawStr = name.into();

    let strings = vec![
        raw_str.percent_encode().to_string(),
        raw_str.html_escape().to_string(),

        // Uri::percent_encode(raw_str).to_string(), // this is in Rocket 4.0... we should support it, even if it's removed. Just in case.
    ];

    let encoded = format!("{:?}",
        strings
    );
    encoded // FP
}

#[get("/?<name>")]
fn sanitized_hello(name: &str) -> (ContentType, String) {
    //let uri: Origin = Origin::parse(name).expect("valid URI");
    let raw_str: &RawStr = name.into();

    let strings = vec![
        raw_str.percent_encode().to_string(),
        raw_str.html_escape().to_string(),

        // Uri::percent_encode(raw_str).to_string(), // this is in Rocket 4.0... we should support it, even if it's removed. Just in case.
    ];

    let encoded = format!("{:?}",
        strings
    );
    (ContentType::HTML, encoded) // FP
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
}


#[get("/?<name>")]
fn tera_template(name: String) -> Template {
    Template::render("tera_test", &TemplateContext { // TemplateContext is just some struct here ^^^^
        title: "Hello",
        name: Some(name),
    }) // flow to tera_test template
}

#[get("/?<name>")]
fn hbs_template(name: String) -> Template {
    Template::render("hbs_test", &TemplateContext {
        title: "Hello",
        name: Some(name), // flow to hbs_test template - only XSS if renders inside {{{ }}} , but sanitized in {{ }}
    }) 
}


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SomeStruct {
    name: Option<String>,
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![hello])
        .mount("/print_file_hello", routes![print_file_hello])
        .mount("/plain_hello", routes![plain_hello])
        .mount("/sanitized_hello", routes![sanitized_hello])
        .mount("/contenttype_hello", routes![contenttype_hello])
        .mount("/rawhtml_hello", routes![rawhtml_hello])
        .mount("/rawxml_hello", routes![rawxml_hello])
        .mount("/contenttype_xml_hello", routes![contenttype_xml_hello])
        .mount("/hbs_test", routes![hbs_template])
        .mount("/tera_test", routes![tera_template])
        .attach(Template::fairing())
}

