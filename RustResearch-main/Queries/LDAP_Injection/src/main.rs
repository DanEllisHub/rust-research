#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;
extern crate tokio_core;

#[cfg(test)]
mod tests;
mod task;

use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::serde::Serialize;
use rocket::form::Form;
use rocket::fs::{FileServer, relative};

use rocket_dyn_templates::Template;

use crate::task::{Task, Todo};

use std::error::Error;

// use futures::Future;
use tokio_core::reactor::Core;
use ldap3::{LdapConn, Scope, SearchEntry, LdapConnAsync};

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Context {
    flash: Option<(String, String)>,
    tasks: Vec<Task>
}

impl Context {
    pub async fn err<M: std::fmt::Display>(conn: &DbConn, msg: M) -> Context {
        Context {
            flash: Some(("error".into(), msg.to_string())),
            tasks: Task::all(conn).await.unwrap_or_default()
        }
    }

    pub async fn raw(conn: &DbConn, flash: Option<(String, String)>) -> Context {
        match Task::all(conn).await {
            Ok(tasks) => Context { flash, tasks },
            Err(e) => {
                error_!("DB Task::all() error: {}", e);
                Context {
                    flash: Some(("error".into(), "Fail to access database.".into())),
                    tasks: vec![]
                }
            }
        }
    }
}

#[post("/", data = "<todo_form>")]
async fn new(todo_form: Form<Todo>, conn: DbConn) -> Flash<Redirect> {
    let todo = todo_form.into_inner();
    if todo.description.is_empty() {
        Flash::error(Redirect::to("/"), "Description cannot be empty.")
    } else if let Err(e) = Task::insert(todo, &conn).await {
        error_!("DB insertion error: {}", e);
        Flash::error(Redirect::to("/"), "Todo could not be inserted due an internal error.")
    } else {
        Flash::success(Redirect::to("/"), "Todo successfully added.")
    }
}

#[put("/<id>")]
async fn toggle(id: i32, conn: DbConn) -> Result<Redirect, Template> {
    match Task::toggle_with_id(id, &conn).await {
        Ok(_) => Ok(Redirect::to("/")),
        Err(e) => {
            error_!("DB toggle({}) error: {}", id, e);
            Err(Template::render("index", Context::err(&conn, "Failed to toggle task.").await))
        }
    }
}

#[delete("/<id>")]
async fn delete(id: i32, conn: DbConn) -> Result<Flash<Redirect>, Template> {
    match Task::delete_with_id(id, &conn).await {
        Ok(_) => Ok(Flash::success(Redirect::to("/"), "Todo was deleted.")),
        Err(e) => {
            error_!("DB deletion({}) error: {}", id, e);
            Err(Template::render("index", Context::err(&conn, "Failed to delete task.").await))
        }
    }
}

#[get("/")]
async fn index(flash: Option<FlashMessage<'_>>, conn: DbConn) -> Template {
    let flash = flash.map(FlashMessage::into_inner);
    Template::render("index", Context::raw(&conn, flash).await)
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    DbConn::get_one(&rocket).await
        .expect("database connection")
        .run(|conn| { conn.run_pending_migrations(MIGRATIONS).expect("diesel migrations"); })
        .await;

    rocket
}

// Example Rocket route with "name" parameter influencing the filter string of the LDAP search
#[get("/ldap_search/<name>")]
async fn ldap_search(name: String) {
    let mut ldap = LdapConn::new("ldap://ldap.forumsys.com").unwrap();

    let (rs, _res) = ldap.search(
        "dc=example,dc=com",
        Scope::Subtree,
        &format!("(&(OU=scientists)(cn={}))", name),    // Sink
        vec!["*"]
    ).unwrap().success().unwrap();
    for entry in rs {
        println!("{:?}", SearchEntry::construct(entry));
    }
    ldap.unbind().unwrap();
}

// Same as above but the name is now influencing the "base" parameter of the LDAP search
#[get("/ldap_search_base/<name>")]
async fn ldap_search_base(name: String) {
    let mut ldap = LdapConn::new("ldap://ldap.forumsys.com").unwrap();

    let (rs, _res) = ldap.search(
        &format!("dc={}", name),    // Sink
        Scope::Subtree,
        "(&(OU=scientists))",
        vec!["*"]
    ).unwrap().success().unwrap();
    for entry in rs {
        println!("{:?}", SearchEntry::construct(entry));
    }
    ldap.unbind().unwrap();
}

// Same as the first example but now the input is sanitized by the "ldap_escape" function
#[get("/ldap_search_escaped/<name>")]
async fn ldap_search_escaped(name: String) {
    let mut ldap = LdapConn::new("ldap://ldap.forumsys.com").unwrap();

    let (rs, _res) = ldap.search(
        "dc=example,dc=com",
        Scope::Subtree,
        &format!("(&(OU=scientists)(cn={}))", ldap3::ldap_escape(&name)),   // Sanitized by ldap_escape
        vec!["*"]
    ).unwrap().success().unwrap();
    for entry in rs {
        println!("{:?}", SearchEntry::construct(entry));
    }
    ldap.unbind().unwrap();
}

// Same as above but now the input is influencing the "base" parameter and is being sanitized by the "dn_escape" function
#[get("/ldap_search_base_escaped/<name>")]
async fn ldap_search_base_escaped(name: String) {
    let mut ldap = LdapConn::new("ldap://ldap.forumsys.com").unwrap();

    let (rs, _res) = ldap.search(
        &format!("dc={}", ldap3::dn_escape(&name)),   // Sanitized by dn_escape
        Scope::Subtree,
        "(&(OU=scientists))",
        vec!["*"]
    ).unwrap().success().unwrap();
    for entry in rs {
        println!("{:?}", SearchEntry::construct(entry));
    }
    ldap.unbind().unwrap();
}

// Same as the first example but the input now comes from a SQL query result
#[get("/ldap_search_stored/<id>")]
async fn ldap_search_stored(id: i32, conn: DbConn) {
    let mut ldap = LdapConn::new("ldap://ldap.forumsys.com").unwrap();

    // get the todo with the given id
    let todo = Task::get_by_id(id, &conn).await.unwrap().expect("Task not found");
    let name = todo.description;

    println!("Searching for: {}", name);

    let (rs, _res) = ldap.search(
        "dc=example,dc=com",
        Scope::Subtree,
        &format!("(&(OU=scientists)(cn={}))", name),    // Sink
        vec!["*"]
    ).unwrap().success().unwrap();
    for entry in rs {
        println!("{:?}", SearchEntry::construct(entry));
    }
    ldap.unbind().unwrap();
}

// Same as above but sanitized by the "ldap_escape" function
#[get("/ldap_search_stored_sanitized/<id>")]
async fn ldap_search_stored_sanitized(id: i32, conn: DbConn) {
    let mut ldap = LdapConn::new("ldap://ldap.forumsys.com").unwrap();

    // get the todo with the given id
    let todo = Task::get_by_id(id, &conn).await.unwrap().expect("Task not found");
    let name = todo.description;

    println!("Searching for: {}", name);

    let (rs, _res) = ldap.search(
        "dc=example,dc=com",
        Scope::Subtree,
        &format!("(&(OU=scientists)(cn={}))", ldap3::ldap_escape(&name)),    // Sanitized
        vec!["*"]
    ).unwrap().success().unwrap();
    for entry in rs {
        println!("{:?}", SearchEntry::construct(entry));
    }
    ldap.unbind().unwrap();
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .attach(AdHoc::on_ignite("Run Migrations", run_migrations))
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index, ldap_search, ldap_search_async, ldap_search_base, ldap_search_escaped, ldap_search_base_escaped, ldap_search_stored])
        .mount("/todo", routes![new, toggle, delete])
}
