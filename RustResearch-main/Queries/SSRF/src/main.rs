#[macro_use]
extern crate rocket;
use reqwest;
use std::fs;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use tokio::io::AsyncWriteExt;
// bucket = "ori-public-bucket-rust";
// key = "test.txt";

static PATH: &str = "../../file-body.txt";

#[get("/ssrf?<url>&<t>")]
async fn ssrf(url: &str, t: u16) -> String {
    match t {
        // Following line will trigger several results
        1 => send_request_vulnerable(&url).await.expect("ssrf failed"),
        // Following line will trigger several results
        2 => send_request_vulnerable(&get_stored_input()).await.expect("ssrf failed"),
        // Following line will trigger several results
        3 => send_request_improper_sanitizer(&url).await,
        // SAFE - Won't trigger results:
        4 => send_request_sanitized(&url).await,
        _ => "unkown operation".to_string(),
    }
}
async fn send_request_vulnerable(url: &str) -> Result<String, std::fmt::Error> {
    // EXAMPLE: TcpStream - connect()
    let mut stream = TcpStream::connect(url).expect("something went wrong"); // RESULT
    stream.write_all(b"bla bla");

    // EXAMPLE: UdpSocket - connect()
    let socket = std::net::UdpSocket::bind("127.0.0.1:34251").expect("something went wrong");
    socket.connect(&url).expect("something wrong"); // RESULT
    let data = &[65, 66, 66];
    socket.send(data).expect("couldn't send message");

    // EXAMPLE: tokio crate - TcpStream - connect()
    let mut tokio_stream = tokio::net::TcpStream::connect(url)
        .await
        .expect("something wrong"); // RESULT
    tokio_stream.write_all(b"bla bla");

    // EXAMPLE: tokio crate - UdpSocket - connect()
    let tokio_socket = tokio::net::UdpSocket::bind("127.0.0.1:34251")
        .await
        .expect("something wrong");
    tokio_socket.connect(&url).await.expect("something wrong"); // RESULT
    let data = &[65, 66, 66];
    tokio_socket
        .send(data)
        .await
        .expect("couldn't send message");

    // EXAMPLE: reqwest crate - get
    // ==============
    // |||| NOTE ||||
    // ==============
    // The following line will cause compile-time error, to solve it take a look
    // at the "reqwest" crate in the "cargo.toml" file of this project.
    // ============================================================================
    // let body = reqwest::blocking::get(url); // RESULT - the first param of get() - `urlAndParams` is output

    // EXAMPLE: reqwest crate - async get
    let res = reqwest::Client::new()
        .get(url) // RESULT
        .send()
        .await
        .expect("something wrong");

    Ok("aaa".to_string())
}

async fn send_request_improper_sanitizer(url: &str) -> String {
    let improper_sanitizer_1 = format!("https://{}", url);
    let improper_sanitizer_2 = format!("abcd.com{}", url);
    let improper_sanitizer_3 = format!("https://abcd.com{}", url);
    let improper_sanitizer_4 = format!("{}", url);
    let improper_sanitizer_5 = format!("{}{}", "https://checkmarx.com" ,url);
    let improper_sanitizer_6 = reqwest::Url::parse(url)
        .expect("msg")
        .domain() // flow should be kept, not a sanitizer
        .expect("msg")
        .to_string();

    let improper_sanitizer_7 = reqwest::Url::parse(url)
        .expect("msg")
        .path() // flow should be kept, not a sanitizer
        .to_string();

    _ = send_request_vulnerable(improper_sanitizer_1.as_str()); // VULNERABLE
    _ = send_request_vulnerable(improper_sanitizer_2.as_str()); // VULNERABLE
    _ = send_request_vulnerable(improper_sanitizer_3.as_str()); // VULNERABLE
    _ = send_request_vulnerable(improper_sanitizer_4.as_str()); // VULNERABLE
    _ = send_request_vulnerable(improper_sanitizer_5.as_str()); // VULNERABLE
    _ = send_request_vulnerable(improper_sanitizer_6.as_str()); // VULNERABLE
    _ = send_request_vulnerable(improper_sanitizer_7.as_str()); // VULNERABLE

    return "".to_string();
}

async fn send_request_sanitized(url: &str) -> String {
    let sanitized_1 = format!("https://checkmarx.com/{}", url);
    let sanitized_2 = format!("https://checkmarx.com\\{}", url);
    let sanitized_3 = format!("https://checkmarx.com/blabla/{}", url);
    let sanitized_4 = format!("{}/{}", "https://checkmarx.com" ,url);
    let sanitized_5 = format!("{}{}", "https://checkmarx.com/" ,url);

    // These invocation should NOT trigger a result
    // SAFE CALLS:
    _ = send_request_vulnerable(sanitized_1.as_str()).await; // SAFE
    _ = send_request_vulnerable(sanitized_2.as_str()).await; // SAFE
    _ = send_request_vulnerable(sanitized_3.as_str()).await; // SAFE
    _ = send_request_vulnerable(sanitized_4.as_str()).await; // SAFE
    _ = send_request_vulnerable(sanitized_5.as_str()).await; // SAFE

    return "".to_string();
}

fn get_stored_input() -> String {
    let path = "stored.txt".to_string();

    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    return contents;
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ssrf])
}
