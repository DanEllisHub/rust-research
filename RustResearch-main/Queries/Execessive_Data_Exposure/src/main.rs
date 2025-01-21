#[macro_use] extern crate rocket;
use std::string::ToString;
use rocket::serde::{Serialize, Deserialize,  json::Json};
use std::fmt::Debug;
use rocket::{Rocket, Request, Data, Build,};
use rocket::request::FromRequest;


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

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    username: String,
    password: String,
}

#[get("/?<username>&<password>")]
fn helloUser(username: String, password: String) -> Json<User> { // Inputs are attributes of opt, and lang
    let greeting = format!("Hello {} ", username);
    let user = User {
        username: username,
        password: password
    };
    
    Json(user) // JSON output
}

#[get("/?<username>&<password>")]
fn helloUserFP(username: String, password: String) -> Json<String> { // Inputs are attributes of opt, and lang
    let greeting = format!("Hello {} ", username);
    let user = User {
        username: username,
        password: password
    };
    let name = user.username; // sanitized because a property that isn't password is passed, but the rest isn't
    Json(name) // JSON output
}





#[launch]
fn rocket() -> _ {
    rocket::build()    
        .mount("/helloStruct", routes![helloStruct])
        .mount("/", routes![helloUser])
        .mount("/fp", routes![helloUserFP])

}
