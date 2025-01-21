use std::{
    io::{stdout, Read, Write},
    sync::Arc,
};

#[macro_use]
extern crate rocket;

use rocket::form;
use rocket::fs;
use rocket::http;
use serde::de::Error;
use std::string::ToString;

use std::fs::File;
use std::io::{self};

fn read_user_file(id: i32) -> Result<String, io::Error> {
    let mut file = std::fs::File::open(format!("./public/books/book_{}.txt", id))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn handle_result_safe_1(result: Result<String, io::Error>) -> String {
    match result {
        Ok(a) => a,
        Err(e) => "SANITIZER - an error occured and handled by returning a message to the user"
            .to_string(),
    }
}

fn handle_result_safe_2(result: Result<String, io::Error>) -> String {
    result.unwrap_or(
        "SAFE - an error occured and handled by returning a message to the user".to_string(),
    )
}

fn handle_result_safe_3(result: Result<String, io::Error>) -> String {
    result.unwrap_or_default() // SAFE
}

fn handle_result_safe_4(result: Result<String, io::Error>) -> String {
    if let Ok(ret) = result {
        return ret;
    } else if let Err(e) = result {
        return "SANITIZER - an error occured, this will crash the app".to_string();
    }
    let a = result.ok(); // this will ignore the error (error is not handled)
    "".to_string()
}

fn handle_result_safe_5(result: Result<String, io::Error>) -> String {
    if let Ok(ret) = result {
        return ret;
    }

    "".to_string()
}

fn handle_result_weak_1(result: Result<String, io::Error>) -> String {
    result.unwrap()
}

fn handle_result_weak_2(result: Result<String, io::Error>) -> String {
    result.expect("an error occured, this will crash the app")
}

fn handle_result_weak_3(result: Result<String, io::Error>) -> String {
    let s = result.expect_err("an error occured, this will crash the app");
    "".to_string()
}

fn handle_result_weak_4(result: Result<String, io::Error>) -> String {
    let err = result.unwrap_err();
    "".to_string()
}

fn handle_result_weak_5(result: Result<String, io::Error>) -> String {
    let _ = result; // this will ignore the error (error is not handled)
    "".to_string()
}

fn handle_result_weak_6(result: Result<String, io::Error>) -> String {
    _ = result; // this will ignore the error (error is not handled)
    "".to_string()
}

#[get("/?<safe>&<id>")]
fn index(safe: bool, id: i32) -> String {
    let mut string_res: String;
    if safe {
        string_res = handle_result_safe_1(read_user_file(id));
        string_res = handle_result_safe_2(read_user_file(id));
        string_res = handle_result_safe_3(read_user_file(id));
        string_res = handle_result_safe_4(read_user_file(id));
        string_res = handle_result_safe_5(read_user_file(id));
    } else {
        if id == 2 {
            string_res = handle_result_weak_1(read_user_file(id));
        }
        else if id == 3 {
            string_res = handle_result_weak_2(read_user_file(id));
        }
        else if id == 4 {
            string_res = handle_result_weak_3(read_user_file(1));
        }
        else if id == 5 {
            string_res = handle_result_weak_4(read_user_file(1));
        }
        else if id == 6 {
            string_res = handle_result_weak_5(read_user_file(1));
        }
        else if id == 7 {
            string_res = handle_result_weak_6(read_user_file(1));
        }
        else {
            string_res = "wrong id was chosen".to_string();
        }
    }
    string_res
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
