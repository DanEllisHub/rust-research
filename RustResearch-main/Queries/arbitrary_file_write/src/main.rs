#[macro_use] extern crate rocket;
use rocket::fs::FileName;
use rocket::fs::TempFile;
use rocket::futures::TryFutureExt;
use std::path::{Path,PathBuf};
use std::str::FromStr;
use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
use rocket::form::{DataField, Form, FromFormField};
use rocket::http::{ContentType, CookieJar, Status};
use std::borrow::Cow;
use rand::{self, Rng};
use rocket::request::FromParam;
const ID_LENGTH: usize = 3;
const HOST: Absolute<'static> = uri!("http://localhost:8000");
#[derive(UriDisplayPath)]
pub struct PasteId<'a>(Cow<'a, str>);
/// A _probably_ unique paste ID.
/// Returns an instance of `PasteId` if the path segment is a valid ID.
/// Otherwise returns the invalid ID as the `Err` value.
impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;
    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param.chars().all(|c| true)//this line might change to   param.chars().all(|c| c.is_ascii_alphanumeric()) becoming a general sanitizer for string parameter 
            .then(|| PasteId(param.into()))
            .ok_or(param)
    }//
}
impl PasteId<'_> {
    /// Generate a _probably_ unique ID with `size` characters. For readability,
    /// the characters used are from the sets [0-9], [A-Z], [a-z]. The
    /// probability of a collision depends on the value of `size` and the number
    /// of IDs generated thus far.
    pub fn new(size: usize) -> PasteId<'static> {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }
        PasteId(Cow::Owned(id))
    }
    /// Returns the path to the paste in `upload/` corresponding to this ID.
    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
        Path::new(root).join(self.0.as_ref())
    }
}

#[post("/vulnerable?<input_name>", data = "<paste>")]//Result
async fn vulnerable(input_name: &str, paste: Data<'_>) -> std::io::Result<String> {
    paste.open(128.kibibytes()).into_file(input_name).await?;//sink & unsanitized input name and unvalidated file content
    Ok("done_vulnerable_endpoint".to_string())
}
#[post("/suffix_over_ridden?<input_name>", data = "<paste>")]//safe
async fn suffix_over_ridden(input_name: &str, paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    let mut sanitized_name = FileName::new(input_name).as_str().unwrap_or("").to_string();//FileName is the sanitizer
    let mut fileName = format!("{}{}", sanitized_name,".png"); // combin sanitized user prefix with known suffix
    paste.open(128.kibibytes()).into_file(fileName).await?;//Sink - not a Result
    Ok("done_safe_changed_suffix_endpoint".to_string())
}
#[post("/half_sanitized", data = "<paste>")]//input does not influence file name // safe
async fn half_sanitized(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);//arbitrary name given to file 
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "half_sanitized");
    paste.open(128.kibibytes()).into_file(path).await?;//sink digest sanitized input
    Ok("done_safe_endpoint_user_input".to_string())
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE
      POST /
          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content
      GET /<id>
          retrieves the content for the paste with id `<id>`
    "
}
#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<rocket::tokio::fs::File> {
    rocket::tokio::fs::File::open(id.file_path()).await.ok()
}

#[post("/upload", data = "<file>")]
pub async fn upload(mut file: Form<TempFile<'_>>,) -> Status  {//safe random file path provided like the endpoint above but with a different input type
    let filename = PasteId::new(ID_LENGTH);

    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "filename");
    // Move the temp file to a permanent location
    if let Err(e) = file.copy_to(root).await {
        eprintln!("Failed to write file to disk: {}", e);
        return Status::InternalServerError;
    }
    println!("File uploaded successfully.");
    Status::Ok
}

// Define your whitelist of content types for file uploads
static FILE_CONTENT_TYPE_WHITELIST: &[ContentType] = &[
    ContentType::XML,
    ContentType::PDF,
    ContentType::GIF,
];

fn is_valid_content_type(content_type: &ContentType) -> bool {
    FILE_CONTENT_TYPE_WHITELIST.contains(content_type)
}

#[post("/fileupload/<userid>", data = "<data>")]//safe
fn fileupload(userid: &str, content_type: &ContentType, data: Data<'_>) -> Status
{
  // to get a param from client
  let mut result=format!("UserID:{}<br>",userid);
  // content_type not used here, just for more informations
  result = format!("{}{:?}<br>",result,content_type);
  // aquire all Form field data
    if !is_valid_content_type(content_type) {//This validates content but, content type might be changed with a proxy meaning this validation can be bypassed 
        return Status::InternalServerError
    }
  //some server upload functionality
  //let mut sanitized_name = FileName::new(&userid).as_str().unwrap_or("").to_string();//FileName is the sanitizer
  //let mut fileName = format!("{}{}", sanitized_name,".png"); // combin sanitized user prefix with known suffix
  let mut fileName = format!("{}{}", userid,".png"); // combin user input prefix with known suffix
    data.open(128.kibibytes()).into_file(fileName);//Sink 

  return Status::Ok
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,retrieve,half_sanitized,suffix_over_ridden,
    vulnerable,upload,fileupload])
}
