#[macro_use]
extern crate rocket;
use rocket::http::uncased::{UncasedStr, Uncased};
use rocket::http::uri::Uri;
use rocket::response::{content, status, Redirect};
use rocket::http::{Status, Header, ContentType};
type OtherResponder = ();
type MyType = u8;
use std::borrow::Cow;
use std::io::Cursor;

use rocket::request::Request;
use rocket::response::{self, Response, Responder};

#[derive(Debug, Clone)]
struct MyResponder {
    url: String,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for MyResponder {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {

        let k = self.url;
        print!("\n\n\n\n url: {}\n\n", k);
        Response::build()
            .status(rocket::http::Status { code: 302 })
            .header(Header{name:Uncased::new("location"), value:Cow::from(k)}) // RESULT
            .ok()
    }
}

#[get("/redirect?<url>&<t>")]
async fn redirect(url: &str, t: u16) -> Redirect {
    match t {
        1 => Redirect::to(url.clone().to_string()), // RESULT
        2 => Redirect::found(invalid_sanitizer_1(url.clone())),  // NOTE: INVALID SANITIZER - RESULT
        3 => Redirect::moved(url.clone().to_string()),  // RESULT
        4 => Redirect::permanent(url.clone().to_string()),  // RESULT
        5 => Redirect::temporary(url.clone().to_string()),  // RESULT
        6 => Redirect::to(prefix_sanitizer_1(url.clone())), // SANITIZED
        _ => Redirect::permanent(whitelist_sanitizer_1(url.clone())),  // SANITIZED
    }
}

#[get("/redirect2?<url>")]
fn redirect2(url: &str) -> MyResponder {
    MyResponder {
        url: url.to_string()
    }
}
// THIS IS NOT A VALID SANITIZER
fn invalid_sanitizer_1(url: &str) -> String{
    format!("https://my-safe-url.com{}", url)
}

fn prefix_sanitizer_1(url: &str) -> String{
    format!("https://my-safe-url.com/{}", url)
}

fn whitelist_sanitizer_1(url: &str) -> String{
    if url.eq("https://checkmarx.com") {
        return url.to_string() // sanitized
    }

    "https://google.com".to_string()
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![redirect])
        .mount("/", routes![redirect2])
}