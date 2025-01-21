use std::env;

fn main() {
    let dir = env::temp_dir(); // RESULT - `temp_dir()` is used
    println!("Temporary directory: {}", dir.display());

    // rest of code ...
}
