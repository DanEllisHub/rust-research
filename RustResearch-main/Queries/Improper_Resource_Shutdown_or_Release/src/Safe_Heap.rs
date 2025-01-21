use std::mem::drop;

fn main() {
    let b = Box::new(42);
    // Sanitizer 
    std::mem::drop(b); // `b` is released here
    
    other_actions_not_related_to_the_Box();
}