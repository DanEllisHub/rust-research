use std::net::TcpStream;
use std::io::prelude::*;
use std::mem::drop;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect");
    stream.write_all(b"Hello, world!")?;
    // Sanitizer
    drop(stream);

    other_actions_not_related_to_the_Network_Connections();
    Ok(())
}