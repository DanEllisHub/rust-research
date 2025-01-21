use std::{
    io::{stdout, Read, Write},
    sync::Arc,
};

#[macro_use]
extern crate rocket;

use rocket::{form, http::{Cookie, CookieJar}, response::Responder, time::format_description::parse};
use rocket::fs;
use rocket::{http, shield::Shield};
use std::string::ToString;
use rocket::response::content::RawHtml;

#[get("/cookie")]
fn give_me_cookie(jar: &CookieJar<'_>) ->  RawHtml<&'static str> {
    let builder = Cookie::build("cookie1_vuln", "value");
    let c1_vuln = Cookie::build("cookie1_vuln", "value").finish();
    let c2_vuln = Cookie::build("cookie2_vuln", "value").path("/").finish();
    let c2_1_vuln = Cookie::build("cookie2_vuln", "value").path("\\").finish();
    let c3_vuln = Cookie::new("cookie3_vuln", "value"); // default is not secure
    let mut c4_vuln = Cookie::new("cookie4_vuln", "value");
    c4_vuln.set_path("/");
    let c5_vuln = Cookie::parse("c5_vuln=value; Path=/; HttpOnly").expect("something went wrong");
    let c6_vuln = Cookie::parse_encoded("c6_vuln=val%20ue; Path=/; HttpOnly").expect("something went wrong");
    let c7_vuln = Cookie::split_parse("c7_1_vuln=value; c7_2_vuln=key%20value").collect::<Vec<_>>();
    // extract the 2 cookies out of c7_vuln
    let c7_1_vuln = c7_vuln[0].clone().unwrap(); // unsafe cookie
    let c7_2_vuln = c7_vuln[1].clone().unwrap(); // unsafe cookie


    jar.add(c1_vuln); // sink - result
    jar.add(c2_vuln); // sink - result
    jar.add(c2_1_vuln); // sink - result
    jar.add(c3_vuln); // sink - result
    jar.add(c4_vuln); // sink - result
    jar.add(c5_vuln); // sink - result
    jar.add(c6_vuln); // sink - result
    jar.add(c7_1_vuln); // sink - result
    jar.add(c7_2_vuln); // sink - result

    let c1_safe = Cookie::build("c1_safe", "value").secure(true).finish();
    let mut c2_safe = Cookie::new("c2_safe", "value");
    c2_safe.set_secure(true);
    let c3_safe = Cookie::parse("c3_safe=value; Path=/; Secure").expect("something went wrong");
    
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
