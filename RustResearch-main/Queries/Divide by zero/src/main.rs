use std::{io::{stdout, Read, Write}, sync::Arc};

#[macro_use] extern crate rocket;

use rocket::http;
use rocket::form;
use rocket::fs;
use std::string::ToString;


#[get("/avg1?<total>&<count>")]
fn avg1(total:i32, count:i32) -> String {
    let avg = total / count; // VULNERABLE
    avg.to_string()
}

#[get("/avg2?<total>&<count>")]
fn avg2(total:i32, count:i32) -> String {
    if count > 0{
        return (total / count).to_string(); // SAFE
    }else{
        return "Cannot divide by zero".to_string();
    }
}

#[get("/avg3?<total>&<count>")]
fn avg3(total:i32, count:i32) -> String {
    if count < 0{
        return (total % count).to_string(); // SAFE
    }
    "".to_string()
}

#[get("/avg4?<total>&<count>")]
fn avg4(total:i32, mut count:i32) -> String {
    // write a for loop
    for i in 0..15 {
        count = count - 1;
    }
    return (total / count).to_string(); // VULNERABLE
}

#[get("/avg5?<total>&<count>")]
fn avg5(total:i32, count:i32) -> String {
    if count <= 0{
        return (total / count).to_string(); // VULNERABLE
    }
    "".to_string()
}

#[get("/avg6?<total>&<count>")]
fn avg6(total:i32, count:i32) -> String {
    let avg = total % count; // VULNERABLE
    avg.to_string()
}


#[get("/avg7?<total>&<count>")]
fn avg7(total:i32, count:i32) -> String {
    if count == 0 { // SANITIZER
        "Cannot divide by zero".to_string()
    }else{
        let avg = total / count; // SAFE
        avg.to_string()
    }
}

#[get("/avg8?<total>&<count>")]
fn avg8(total:i32, count:i32) -> String {
    let mut avg;
    if count == 0 {
        avg = total / (count + 1); // SAFE
    }else{
        avg = total / count; // SAFE
    }
    avg.to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![avg1])
        .mount("/", routes![avg2])
        .mount("/", routes![avg3])
        .mount("/", routes![avg4])
        .mount("/", routes![avg5])
        .mount("/", routes![avg6])
        .mount("/", routes![avg7])       
        .mount("/", routes![avg8])       
}
