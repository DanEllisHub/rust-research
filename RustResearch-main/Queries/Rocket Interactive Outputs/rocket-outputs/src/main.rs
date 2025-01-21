#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
use rocket::time::Date;
use rocket::http::{Status, ContentType};
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
use rocket::response::content::RawHtml;

#[get("/?<name>")] // Note - .. when handling structs
fn hello(name: String) -> String { // Inputs are attributes of opt, and lang
    let greeting = format!("Hello {} ", name);
    greeting // output; can also be a return
    // return greeting // this is just shorthand
}

#[get("/?<name>")] // Note - .. when handling structs
fn helloHtml(name: String) -> (ContentType, String) { // Inputs are attributes of opt, and lang
    let greeting = format!("Hello {} ", name);
    (ContentType::HTML, greeting) // 2nd parameter is output; can also be a return
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SomeStruct {
    state: bool,
    text: String
}

#[get("/?<name>")]
fn helloStruct(name: String) -> Json<SomeStruct> { // Inputs are attributes of opt, and lang
    let greeting = format!("Hello {} ", name);
    let some_struct = SomeStruct {
        state: true, 
        text : greeting
    }; 
    Json(some_struct) // JSON output
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
}

#[get("/?<name>")]
fn tera_template(name: String) -> Template {
    Template::render("tera_test", &TemplateContext {
        title: "Hello",
        name: Some(name),
    }) // flow to tera_test template
}

#[get("/?<name>")]
fn hbs_template(name: String) -> Template {
    Template::render("hbs_test", &TemplateContext {
        title: "Hello",
        name: Some(name),
    }) // flow to hbs_test template
}


#[get("/?<name>&<value>")]
fn addcookie(cookies: &CookieJar<'_>, name: String, value: String) -> String {
    let msg = format!("Added {}={} to cookie", name, value);
    cookies.add(Cookie::new(name.clone(), value.clone())); // Output - both name and value
    let cookie = Cookie::build(name, value) // Output - again both name and value
    // NONE of the following are likely outputs - they sould be excluded,
    // to reduce FPs on cookie creation, in case these fields are affected by user input
    .domain("localhost")
    .path("/")
    .secure(true)
    .http_only(true)
    .finish(); 
    return msg
}

#[catch(404)]
fn hello_not_found(req: &Request<'_>) -> RawHtml<String> {
    RawHtml(format!("\
        <p>Sorry, but '{}' is not a valid path!</p>\
        <p>Try visiting /hello/&lt;name&gt;/&lt;age&gt; instead.</p>",
        req.uri()) // output
    ) 
}

#[post("/login/<name>")]
fn login(name: &str) -> Result<&'static str, Flash<Redirect>> {
    if name == "special_user" {
        Ok("Hello, special user!")
    } else {
        let errormsg = format!("Invalid username {}", name)
        Err(Flash::error(Redirect::to(uri!(index)), errormsg)) // errormsg is a cookie output
    }
}

struct Counter {
    get: AtomicUsize,
    post: AtomicUsize,
}

#[rocket::async_trait]
impl Fairing for Counter {
    // This is a request and response fairing named "GET/POST Counter".
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Request | Kind::Response
        }
    }

    // Increment the counter for `GET` and `POST` requests.
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        match request.method() {
            Method::Get => self.get.fetch_add(1, Ordering::Relaxed),
            Method::Post => self.post.fetch_add(1, Ordering::Relaxed),
            _ => return
        };
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // Don't change a successful user's response, ever.
        if response.status() != Status::NotFound {
            return
        }

        // Rewrite the response to return the current counts.
        if request.method() == Method::Get && request.uri().path() == "/counts" {
            let get_count = self.get.load(Ordering::Relaxed);
            let post_count = self.post.load(Ordering::Relaxed);
            let body = format!("Get: {}\nPost: {}", get_count, post_count);
            let header = format!("Some header data influenced by inputs {:?}", request.uri().query());
            response.set_status(Status::Ok);
            response.set_header(Header::new("AAAA",header)); //output
            response.set_sized_body(body.len(), Cursor::new(body)); //output
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/addcookie", routes![addcookie])
        .mount("/hbs_test", routes![hbs_template])
        .mount("/tera_test", routes![tera_template])
        .mount("/hello", routes![hello])
        .mount("/helloHtml", routes![helloHtml])
        .mount("/helloStruct", routes![helloStruct])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("/static")))
        .attach(AdHoc::on_request("Check out this silly thing", |req, _| { // first param here is input(request object), second is output (response)
            Box::pin(async move {
                println!("    => Incoming request: {}", req.uri().path() );
            })
        }))
        .attach(AdHoc::on_response("Response Rewriter", |req, res| { // first param here is input(request object), second is output (response)
            Box::pin(async move {
                if req.method() == Method::Get && req.uri().path() == "/hello" {
                    let body = format!("Request {:?}", req.uri().query());
                    let header = format!("Some header data influenced by inputs {:?}", req.uri().query());
                    res.set_status(Status::Ok);
                    res.set_header(ContentType::Plain);
                    res.set_header(Header::new("AAAA",header));// Also output
                    res.set_sized_body(body.len(), Cursor::new(body)); // Also output, second parameter
                }
            })
            
        }))
        .register("/", catchers![hello_not_found])
}