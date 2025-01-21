#![feature(absolute_path)]

use handler::{
    serve_file_pathBuf,
    path_flag_pathBuf_sanitizers,
    path_flag_pathBuf,
    serve_absolute_hardcoded,
    path_flag_Isabsolute,
    path_canonicalize_absolute_starts,
    path_flag_IsRelative,
    pathBufdecodedOnce,
    pathBufdecodedDouble,
};
#[macro_use]
extern crate rocket;
extern crate core;
mod handler;
mod model;
mod response;
#[launch]
fn rocket() -> _ {
    let app_data = model::AppState::init();
    println!("{}", "🚀 The server is ready to accept requests");
    rocket::build().manage(app_data).mount(
        "/",
        routes![
            serve_file_pathBuf,
            path_flag_pathBuf,
            path_flag_pathBuf_sanitizers,
            path_flag_Isabsolute,
            path_canonicalize_absolute_starts,
            path_flag_IsRelative,
            pathBufdecodedOnce,
            pathBufdecodedDouble,
        ],
    )
        .mount(
            "/encode/",
            routes![
           // serve_file_pathBuf_encode,
                serve_absolute_hardcoded,
            ]
        )
}