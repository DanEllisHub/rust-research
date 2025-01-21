use std::{fs, path};
use std::path::{Path, PathBuf};
use path_clean::{clean, PathClean};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::response::GenericResponse;
use rocket::fs::NamedFile;

const ALLOWED_DIRS: [&str; 3] = ["public", "uploads", "/Users/bengamliel/my/Rust/pathTraversal",];  // Whitelist of allowed directories
fn is_path_allowed(path: &Path) -> bool {
    // Check if the path is within an allowed directory
    ALLOWED_DIRS.iter().any(|allowed_dir| path.starts_with(allowed_dir))
}
fn url_decode(text: String) -> String {
    urlencoding::decode(&text)
        .expect("something went wrong")
        .to_string()
}

pub fn include_starts_with(static_dir: &Path,file_path: PathBuf,sanitizer: bool) -> Result<Json<GenericResponse>, Status>{
    if sanitizer==true {
        if file_path.starts_with(static_dir) {//second part sanitizer, alone is insufficient
            let contents = fs::read_to_string(&file_path).expect("Should have been able to read the file");//Sink!
            let response_json = GenericResponse {
                status: "success".to_string(),
                message: contents.to_string(),
            };
            Ok(Json(response_json))
        } else {
            let response_json = GenericResponse {
                status: "failed the starts_with test".to_string(),
                message: "base dir dosen't match ".to_string(),
            };
            Ok(Json(response_json))
        }
    }
    else{
    let contents = fs::read_to_string(&file_path).expect("Should have been able to read the file");//Sink!
    let response_json = GenericResponse {
        status: "without starts with validation".to_string(),
        message: contents.to_string(),
    };
    Ok(Json(response_json))
}}
#[get("/f/<path..>")] //Payload: http://127.0.0.1:8000/e/%252E%252E%252Fsrc/main.rs
pub async fn pathBufdecodedOnce(path: PathBuf) -> Option<NamedFile> {//Vulnerable FN
    // PathBuf automatically sanitized and decoded once, hence, the explicit
    // url-decoding in the line below is actually double decoding
    let decoded_path = url_decode(path.display().to_string());//desanitizer- path buf decode, and then manually decoded by the developer thus bypass the encoding sanitizer
    print!("{}", decoded_path);
    NamedFile::open(Path::new("./static/").join(path.display().to_string())).await.ok()//sink
}
#[get("/e/<path..>")]
pub async fn pathBufdecodedDouble(path: PathBuf) -> Option<NamedFile> {//Safe - FromSeg decode once canonicalize and add a base dir, sanitized.
    NamedFile::open(Path::new("./static/").join(decoded_path))//Sink
        .await
        .ok()
}

#[get("/f/path?<input>")]//Safe
pub async fn path_flag_IsRelative(input: String) -> Result<Json<GenericResponse>, Status> {
    let result = Path::new(input.as_str()).is_relative();//Sanitizer
    if result == true {//this is sanitizing for absolute path traversal (relative == True)
        let contents = fs::read_to_string(&input).expect("Should have been able to read the file");//Sink!
        let response_json = GenericResponse {
            status: "user given path is abs".to_string(),
            message: contents.to_string(),
        };
        Ok(Json(response_json))
    }
    else {//this is sanitizing for relative path traversal (relative == False)
        let response_json = GenericResponse {
            status: "result == false *** user input path is not absoulte ***".to_string(),
            message: contents.to_string(),
        };
        Ok(Json(response_json))
    }
}
//IsABSOLUTE  path to be tested
#[get("/d/path?<input>")]// endpoint ready, just modify
pub async fn path_flag_Isabsolute(input: String) -> Result<Json<GenericResponse>, Status> {
    let result = Path::new(input.as_str()).is_absolute();
    if result == true {//this is sanitizing for relative path traversal (absolute == True)
        let contents = fs::read_to_string(&input).expect("Should have been able to read the file");//Sink!
        let response_json = GenericResponse {
            status: "user given path is abs".to_string(),
            message: contents.to_string(),
        };
        Ok(Json(response_json))
    }
    else {//this is sanitizing for absolute path traversal (absolute == False)
        let response_json = GenericResponse {
            status: "result == false *** user input path is not absoulte ***".to_string(),
            message: contents.to_string(),
        };
        Ok(Json(response_json))
    }
}
//Absoulte - gets user input assuming its a full path, canonicalize it and then test it to see if the prefix is safe
#[get("/c/path?<input>&<flag>")]// Done - Both relative(88) and absolute sanitized
pub async fn path_canonicalize_absolute_starts(input: String,flag: bool) -> Result<Json<GenericResponse>, Status> {
    let base_dir = Path::new("/Users/bengamliel/my/Rust/pathTraversal");
    let mut _path = Path::new(input.as_str());
    println!("the input path is :{}", _path.display());
    let  canonicalize_file_path  = _path.canonicalize().unwrap();
    println!("the canonicalize path is :{}", canonicalize_file_path.display());
    let absolute = path::absolute(canonicalize_file_path);//Santizer for Relative path traversal
    println!("the absolute path is :{:?}", &absolute.as_ref());
    return include_starts_with(base_dir,absolute.unwrap(),flag);
    //
}
#[get("/b/pathBuf?<input>&<flag>")]// vulnerable
pub async fn path_flag_pathBuf_sanitizers(input: String,flag: bool) -> Result<Json<GenericResponse>, Status> {
    let static_dir = Path::new("/Users/bengamliel/my/Rust/pathTraversal");
    let mut _path = Path::new(input.as_str());
    println!("the input path is :{}", _path.display());
    let mut sanitized_file_path  = _path.clean().unwrap();//path.clean() insufficient sanitizer, also insufficient with starts with afterward
    println!("after sanitize path is: {}", sanitized_file_path.as_path().display().to_string());
    let mut file_path = static_dir.join(sanitized_file_path);
    println!("after join final path is: {}", file_path.display());
    return include_starts_with(static_dir,file_path,flag);
}

#[get("/a/pathBuf?<input>&<flag>")]//path-string//DONE
pub async fn path_flag_pathBuf(input: String,flag: bool) -> Result<Json<GenericResponse>, Status> {
    let static_dir = Path::new("/Users/bengamliel/my/Rust/pathTraversal");
    let mut _path = Path::new(input.as_str());
    println!("the input path is :{}", _path.display());
    let mut sanitized_file_path  = _path.canonicalize().unwrap();//half sanitized, still vulnerable if start with dosen't validate path result
    println!("after sanitize path is: {}", sanitized_file_path.as_path().display().to_string());
    let mut file_path = static_dir.join(sanitized_file_path);
    println!("after join final path is: {}", file_path.display());
    return include_starts_with(static_dir,file_path,flag);
}//http://127.0.0.1:8000/a/pathBuf?input=temp.txt&flag=false
//http://127.0.0.1:8000/a/pathBuf?input=../../../../../etc/passwd&flag=false -- vulnerable
//http://127.0.0.1:8000/a/pathBuf?input=../../../../../etc/passwd&flag=true -- safe
#[get("/a/<file..>")]//vulnerable double decoding
pub async fn serve_file_pathBuf(file: PathBuf) -> Option<rocket::fs::NamedFile> {
    let string_path = file.display().to_string();
    println!("decoded once: {}", string_path);
    let decoded = urlencoding::decode(&string_path).expect("something went wrong");
    let k = decoded.to_string();
    println!("decoded twice: {}", k);
    NamedFile::open(Path::new("/").join(k)).await.ok()//sink
}

#[get("/normal")]//POC - path hardcoded
pub async fn serve_absolute_hardcoded() -> Result<Json<GenericResponse>, Status> {
    let contents = fs::read_to_string("../../../../../../etc/passwd")//sink,function argument -> source; RESULT
        .expect("Should have been able to read the file");

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: contents.to_string(),
    };
    Ok(Json(response_json))
}