use std::{
    io::{stdout, Read, Write},
    sync::Arc,
};

#[macro_use]
extern crate rocket;

use rocket::fs;
use rocket::response::content::RawHtml;
use rocket::{
    form,
    http::{Cookie, CookieJar},
    response::Responder,
    time::format_description::parse,
};
use rocket::{http, shield::Shield};
use std::string::ToString;

#[get("/cookie")]
fn give_me_cookie(jar: &CookieJar<'_>) -> RawHtml<&'static str> {
    let c1_vuln = Cookie::build("cookie1_vuln", "value").same_site(http::SameSite::None).secure(true).finish();
    let mut c4_vuln = Cookie::new("cookie4_vuln", "value");
    c4_vuln.set_same_site(http::SameSite::None);
    c4_vuln.set_secure(true);
    let c5_vuln = Cookie::parse("c3_vuln=value; SameSite=None; Secure").expect("something went wrong");
    jar.add(c1_vuln); // sink - result
    jar.add(c4_vuln); // sink - result
    jar.add(c5_vuln); // sink - result


    let c1_safe = Cookie::build("c1_safe", "value").http_only(true).finish();
    let mut c2_safe = Cookie::new("c2_safe", "value");
    c2_safe.set_http_only(true);
    let c3_safe = Cookie::parse("c3_safe=value; Path=/; HttpOnly").expect("something went wrong");
    jar.add(c1_safe); // sink - NOT a result, this is safe
    jar.add(c2_safe); // sink - NOT a result, this is safe
    jar.add(c3_safe); // sink - NOT a result, this is safe
    RawHtml(r#"<h1>cookies for all</h1>"#)
}

#[get("/")]
fn home() -> String {
    "done".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Shield::new())
        .mount("/", routes![home])
        .mount("/", routes![give_me_cookie])
}
