use std::net::TcpStream;
use std::io::prelude::*;
fn main() -> std::io::Result<()> {
    // Sink
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect");
    stream.write_all(b"Hello, world!")?;
    other_actions_not_related_to_the_Network_Connections();
    Ok(())
}