#[macro_use]
extern crate rocket;
use std::collections::HashMap;
use std::fs;


#[get("/env2?<interactive_input>")]
fn tokio_env_injection(interactive_input: String) -> String {
    use tokio::process::Command;
    let stored_input = get_stored_input();
    
}

fn get_stored_input() -> String{
    let path = "stored.txt".to_string();

    let contents = fs::read_to_string(path)
    .expect("Should have been able to read the file");

    return contents;
}



fn main() {
    println!("Hello, world!");
}
