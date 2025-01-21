#[macro_use] extern crate rocket;

use rocket::time::Date;
use rocket::http::{Status, ContentType};
use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};
use rocket::fs::{FileServer, TempFile, relative};
use rocket_dyn_templates::Template;
use rocket::{State, Shutdown};
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
use rocket::http::{CookieJar,Method};
use rocket::http::uri::Host;
use rocket::request::FromRequest;
use rocket::route::Route;


#[cfg(test)] mod tests;

#[derive(FromFormField)]
enum Lang {
    #[field(value = "en")]
    English,
    #[field(value = "ru")]
    #[field(value = "ру")]
    Russian
}

#[derive(FromForm)]
struct Options<'r> {
    emoji: bool,
    name: Option<&'r str>,
}

// No input
#[get("/world")]
fn world() -> (ContentType,&'static str) {
    (ContentType::HTML,"Hello, world!")
}

// No input
#[get("/мир")]
fn mir() -> &'static str {
    "Привет, мир!"
}

// Path parameter inputs
//   http://127.0.0.1:8000/wave/Rocketeer/100
#[get("/<name>/<age>")]
fn wave(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

#[get("/?<name>&<age>")]
fn wave2(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

#[derive(FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User<'r> {
    age: u8,
    name: Option<&'r str>,
}

// ?user.name=john&user.age=5
#[get("/?<user>")]
fn wave3(user: User) -> String {
    format!("👋 Hello, {} year old named {}!", user.age, user.name.unwrap())
}

#[post("/", data="<user>")]
fn wave4(user: Form<User>) -> String {
    format!("👋 Hello, {} year old named {}!", user.age, user.name.unwrap())
}

#[post("/", data="<user>")]
fn wave4json(user: Json<User>) -> String {
    format!("👋 Hello, {} year old named {}!", user.age, user.name.unwrap())
}
// GET Parameter inputs
// Note: without the `..` in `opt..`, we'd need to pass `opt.emoji`, `opt.name`.
//
// Try visiting:
//   http://127.0.0.1:8000/?emoji
//   http://127.0.0.1:8000/?name=Rocketeer
//   http://127.0.0.1:8000/?lang=ру
//   http://127.0.0.1:8000/?lang=ру&emoji
//   http://127.0.0.1:8000/?emoji&lang=en
//   http://127.0.0.1:8000/?name=Rocketeer&lang=en
//   http://127.0.0.1:8000/?emoji&name=Rocketeer
//   http://127.0.0.1:8000/?name=Rocketeer&lang=en&emoji
//   http://127.0.0.1:8000/?lang=ru&emoji&name=Rocketeer
#[get("/?<lang>&<opt..>")] // Note - .. when handling structs
fn hello(lang: Option<Lang>, opt: Options<'_>) -> String { // Inputs are attributes of opt, and lang
    let mut greeting = String::new();
    if opt.emoji {
        greeting.push_str("👋 ");
    }

    match lang {
        Some(Lang::Russian) => greeting.push_str("Привет"),
        Some(Lang::English) => greeting.push_str("Hello"),
        None => greeting.push_str("Hi"),
    }

    if let Some(name) = opt.name {
        greeting.push_str(", ");
        greeting.push_str(name);
    }

    greeting.push('!');
    greeting
}

// Very basic POST
#[derive(FromForm)]
struct MyForm {
    text: String,
}

#[post("/", data = "<myform>")]
fn test_post(myform: Form<MyForm>) -> String{
    return format!("{:?}",myform.text);
}


// POST Parameters and Request body
#[derive(Debug, FromForm)]
struct Password<'v> {
    #[field(validate = len(6..))]
    #[field(validate = eq(self.second))]
    first: &'v str,
    #[field(validate = eq(self.first))]
    second: &'v str,
}

#[derive(Debug, FromFormField)]
enum Rights {
    Public,
    Reserved,
    Exclusive,
}

#[derive(Debug, FromFormField)]
enum Category {
    Biology,
    Chemistry,
    Physics,
    #[field(value = "CS")]
    ComputerScience,
}

#[derive(Debug, FromForm)]
#[allow(dead_code)]
struct Submission<'v> {
    #[field(validate = len(1..))]
    title: &'v str,
    date: Date,
    #[field(validate = len(1..=250))]
    r#abstract: &'v str,
    #[field(validate = ext(ContentType::PDF))]
    file: TempFile<'v>,
    #[field(validate = len(1..))]
    category: Vec<Category>,
    rights: Rights,
    ready: bool,
}

#[derive(Debug, FromForm)]
#[allow(dead_code)]
struct Account<'v> {
    #[field(validate = len(1..))]
    name: &'v str,
    password: Password<'v>,
    #[field(validate = contains('@').or_else(msg!("invalid email address")))]
    email: &'v str,
}

#[derive(Debug, FromForm)]
#[allow(dead_code)]
struct Submit<'v> {
    account: Account<'v>,
    submission: Submission<'v>,
}


#[get("/")]
fn index() -> Template {
    Template::render("index", &Context::default())
}

// POST a multi-part file using structs - 

// NOTE: We use `Contextual` here because we want to collect all submitted form
// fields to re-render forms with submitted values on error. If you have no such
// need, do not use `Contextual`. Use the equivalent of `Form<Submit<'_>>`.
#[post("/", data = "<form>")]
fn submit<'r>(form: Form<Contextual<'r, Submit<'r>>>) -> (Status, Template) {
    let template = match form.value {
        Some(ref submission) => {
            println!("submission: {:#?}", submission);
            Template::render("success", &form.context)
        }
        None => Template::render("index", &form.context),
    };
    println!("aaaa {}", form.context.status());
    (form.context.status(), template)
}

//  Messaging via eventstream
#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

// rocket::State,
#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

/// Receive a message from a form submission and broadcast it to any receivers.
#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    // A send 'fails' if there are no active subscribers. That's okay.
    let _res = queue.send(form.into_inner());
}



// test; no inputs
#[get("/")]
fn eventstream_test() -> EventStream![] {
    EventStream! {
        for i in 0..50 {
            yield Event::retry(Duration::from_secs(10));
            yield Event::data(format!("{}", i)).id("cat").event("bar");
            yield Event::comment("silly boy");
        }
    }
}

// Request object attributes are all inputs, and request itself is an input as well
// Note that not all methods can recieve Request objects like these, and require specific implementations
// Also note that not all Request objects are inputs, some of them are for HTTP Clients
#[catch(404)]
fn not_found(req: &rocket::Request) -> String {
    let header_map = req.headers();
    let cookies = req.cookies();
    format!("{}\r\n{}\r\n{}\r\n{} ",
        req.to_string(),
        req.uri(),
        format!("{:?}",cookies),
        format!("{:?}",header_map),
        

    //    req.headers().to_string(),
        //req.segments(0..).to_string()
    )
}

// Inputs from cookies
#[get("/")]
fn test_from_cookies(cookies: &CookieJar) -> String { // Cookies is itself an input
    // msg is the value of a cookie called "message", retrieved with cookies.get("message")
    let msg = cookies.get("message").map(|crumb| format!("Message: {}", crumb.value())); 
    return format!("All Cookies: {}\r\n\r\n {}", format!("{:?}",cookies), msg.unwrap());
}


// Additional critical types that can be tainted with user inputs and should be considered inputs
#[get("/")]
fn test_from_request_stuff(route: &Route, host: &Host) -> String {
    return format!("{} {}",route.to_string(), host.to_string());
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/hello", routes![world, mir])
        .mount("/wave", routes![wave])
        .mount("/wave2", routes![wave2])
        .mount("/wave3", routes![wave3])
        .mount("/wave4", routes![wave4])
        .mount("/wave4json", routes![wave4json])
        .mount("/form", routes![index, submit])
        .mount("/test_post", routes![test_post])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("/static")))
        .attach(AdHoc::on_request("Check out this silly thing", |req, _| { // first param here is input(request object), second is output (response)
            Box::pin(async move {
                println!("    => Incoming request: {}", req.uri().path() );
            })
        }))
        .attach(AdHoc::on_response("Response Rewriter", |req, res| { // first param here is input(request object), second is output (response)
            Box::pin(async move {
                if req.uri().path() == "/rewrite_me" {
                    println!("    => Rewriting response body.");
                    res.set_sized_body(None, Cursor::new("Hello, fairings!"));
                }
            })
        }))

        
        .manage(channel::<Message>(1024).0)
        .mount("/events", routes![post, events])
        .mount("/eventstream_test", routes![eventstream_test])
        .mount("/test_from_cookies", routes![test_from_cookies])
        .mount("/test_from_request_stuff", routes![test_from_request_stuff])
        .register("/", catchers![not_found])
}
