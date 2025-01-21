use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
fn main() {
    // Sink 
    let mut file = File::create("file.txt").expect("Unable to create file");
    file.write_all(b"ok").expect("Unable to write data");

    // Sink
    let mut file_open = File::open("foo.txt")?;
    let mut contents = String::new();
    file_open.read_to_string(&mut contents)?;

    other_actions_not_related_to_the_file_handlers();
}