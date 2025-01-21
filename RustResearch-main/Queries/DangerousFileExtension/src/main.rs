#[macro_use] extern crate rocket;
use rocket::futures::TryFutureExt;
use std::path::{Path,PathBuf};
use std::str::FromStr;
use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
use rocket::form::{DataField, Form, FromFormField};
use rocket::http::{ContentType, CookieJar, Status};
use std::borrow::Cow;
use rocket::request::FromParam;




// Function to validate the file extension
fn validate_extension(file_name: &str, allowed_extensions: &[&str]) -> bool {
    Path::new(file_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| allowed_extensions.contains(&ext))
        .unwrap_or(false)
}

// Route to handle file uploads
#[post("/safe?<file_name>", data = "<paste>")]//input // safe
async fn safe(file_name: &str,paste: Data<'_>) -> std::io::Result<String>{
 
    // Define allowed extensions
    let allowed_extensions = ["jpg", "png", "txt"];
    if !validate_extension(file_name, &allowed_extensions) {//sanitizer
       return Ok("Invalid file extension".to_string());
    }
    paste.open(128.kibibytes()).into_file(file_name).await?;//sink & result
    return Ok("File uploaded successfully".to_string());
}


#[post("/vulnerable?<input_name>", data = "<paste>")]
async fn vulnerable(input_name: &str, paste: Data<'_>) -> std::io::Result<String> {
    paste.open(128.kibibytes()).into_file(input_name).await?;//sink & result
    Ok("done_vulnerable_endpoint".to_string())
}




#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![safe,vulnerable])
}


