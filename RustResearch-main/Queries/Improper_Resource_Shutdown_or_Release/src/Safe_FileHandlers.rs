use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
fn main() {
    let mut file = File::create("file.txt").expect("Unable to create file");
    file.write_all(b"ok").expect("Unable to write data");

    // Sanitizer
    drop(file);

    // Example 2 
    let file_opened = File::open("file.txt").expect("Unable to open file");
    let mut reader = BufReader::new(file_opened);
    let mut text = String::new();
    reader.read_to_string(&mut text).expect("Unable to read data");

    // Sanitizer
    drop(reader);

    other_actions_not_related_to_the_file_handlers();
}