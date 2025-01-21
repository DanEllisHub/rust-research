#[macro_use] extern crate rocket;
mod paste_id;
use std::path::Path;
use paste_id::PasteId;
use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
use rocket::form::{DataField, Form, FromFormField};
use rocket::http::{ContentType, CookieJar, Status};


const ID_LENGTH: usize = 3;
const HOST: Absolute<'static> = uri!("http://localhost:8000");
/* 
pub struct File<'v> {
    file_name: Option<&'v FileName>,
    content_type: ContentType,
    data: Vec<u8>,
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for File<'v> {
    async fn from_data(field: DataField<'v, '_>) -> rocket::form::Result<'v, Self> {
        let stream = field.data.open(u32::MAX.bytes());
        let bytes = stream.into_bytes().await?;
        Ok(File {
            file_name: field.file_name,
            content_type: field.content_type,
            data: bytes.value,
        })

    }
}
#[derive(FromForm)]
pub struct UploadRequest<'r> {
    file: File<'r>,
}

#[post("/testupload",content_type= data = "<req>")]//input
pub async fn testupload(content_type: &ContentType,req: Form<UploadRequest<'_>>,
    cookies: &CookieJar<'_>,) -> Result<RawHtml<String>, Status> {
    Ok(RawHtml(format!("file: {}, md5: {}, content-type: {} / {}",
                       req.file.file_name.unwrap().as_str().unwrap_or("Frack") ,
                       &req.file.data.md5(),
                       content_type,
                       req.file.content_type, 
    )))
}
 
#[post("/", data = "<paste>")]//vulnerable endpoint -  accept any file type 
// & no sanitization with the from param 
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;//sink
    Ok(uri!(HOST, retrieve(id)).to_string())
}*/
//Dms vulernable 
#[post("/vulnerable?<input_name>", data = "<paste>")]//input
async fn vulnerable(input_name: &str, paste: Data<'_>) -> std::io::Result<String> {
    paste.open(128.kibibytes()).into_file(input_name).await?;//sink & result
    Ok("done_vulnerable_endpoint".to_string())
}
#[post("/suffix_over_ridden?<input_name>", data = "<paste>")]//input //safe example
async fn suffix_over_ridden(input_name: &str, paste: Data<'_>) -> std::io::Result<String> {
    let mut fileName = format!("{}.{}", input_name, "png");// combin user prefix with safe suffix
    paste.open(128.kibibytes()).into_file(fileName).await?;//changed file extension to safe one 
    Ok("done_safe_changed_suffix_endpoint".to_string())
}
//DMS

#[post("/safe?<filename>", data = "<paste>")]//input // safe
async fn safe(paste: Data<'_>) -> std::io::Result<String> {
    // Validate the file extension
    let allowed_extensions = ["jpg", "png", "gif"];
    let extension = Path::new(filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");
    
    if !allowed_extensions.contains(&extension) {
        return Err(Status::BadRequest);
    }
    let id = PasteId::new(ID_LENGTH);
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;//sink unvulnerable 
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,retrieve,safe,suffix_over_ridden,vulnerable])
}
