use rocket::outcome::IntoOutcome;
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::{Redirect, Flash};
use rocket::http::{CookieJar, Status};
use rocket::form::Form;

use rocket_dyn_templates::{Template, context};

#[derive(FromForm)]
struct Login<'r> {
    username: &'r str,
    password: &'r str
}

#[derive(Debug)]
struct User{
    role: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
        request.cookies()
            .get_private("role")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|role| User { role })
            .or_forward(Status::Unauthorized)
    }
}

#[macro_export]
macro_rules! session_uri {
    ($($t:tt)*) => (rocket::uri!("/session", $crate::session:: $($t)*))
}

pub use session_uri as uri;

#[get("/")]
fn index(user: User) -> Template {
    Template::render("session", context! {
        user_id: user.role,
    })
}

#[get("/", rank = 2)]
fn no_auth_index() -> Redirect {
    Redirect::to(uri!(login_page))
}

#[get("/login")]
fn login(user: User) -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/login", rank = 2)]
fn login_page(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("login", &flash)
}

#[get("/login_safe")]
fn login_page_safe(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("login_safe", &flash)
}

fn authenticate(username: &str, password: &str) -> bool {
    true
}

#[post("/login?<role>", data = "<login>")]
fn post_login(role: String, jar: &CookieJar<'_>, login: Form<Login<'_>>) -> Result<Redirect, Flash<Redirect>> {
    if authenticate(login.username, login.password) {
        jar.add_private(("role", role)); // RESULT - Querystring influecnes the private cookie
        Ok(Redirect::to(uri!(index)))
    } else {
        Err(Flash::error(Redirect::to(uri!(login_page)), "Invalid credentials."))
    }
}

fn get_role(username: &str) -> String {
    "user".to_string()
}

#[post("/login_sanitized", data = "<login>")]
fn login_sanitized(jar: &CookieJar<'_>, login: Form<Login<'_>>) -> Result<Redirect, Flash<Redirect>> {
    if authenticate(login.username, login.password) {
        let role = get_role(login.username);
        if login.language == "en" || login.language == "fr"{
          jar.add_private(("lang", login.language)); // safe - whitelist        
        }
        jar.add_private(("role", role)); // safe
        Ok(Redirect::to(uri!(index)))
    } else {
        Err(Flash::error(Redirect::to(uri!(login_page)), "Invalid credentials."))
    }
}

#[post("/login_safe", data = "<login>")]
fn post_login_safe(jar: &CookieJar<'_>, login: Form<Login<'_>>) -> Result<Redirect, Flash<Redirect>> {
    if authenticate(login.username, login.password) {
        let role = get_role(login.username); // SANITIZER
        jar.add_private(("role", role)); // SAFE
        Ok(Redirect::to(uri!(index)))
    } else {
        Err(Flash::error(Redirect::to(uri!(login_page)), "Invalid credentials."))
    }
}

#[post("/logout")]
fn logout(jar: &CookieJar<'_>) -> Flash<Redirect> {
    jar.remove_private("role");
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, no_auth_index, login, login_page, post_login, logout, post_login_safe, login_page_safe, login_sanitized]
}
