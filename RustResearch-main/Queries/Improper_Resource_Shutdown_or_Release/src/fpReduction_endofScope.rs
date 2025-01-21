// False Positive 
fn main() {
    let boxed_integer = Box::new(42);
    match process_boxed_integer(&boxed_integer) {
        Ok(result) => println!("Processed result: {}", result),
        Err(error) => eprintln!("Error processing boxed integer: {}", error),
    }
}

fn process_boxed_integer(b: &Box<i32>) -> Result<i32, &'static str> {
    let result = **b + 10;
    Ok(result)
}


// True Positive 
use std::rc::Rc;

fn main() {
    let boxed_integer = Box::new(42);
    
    match process_boxed_integer(&boxed_integer) {
        Ok(result) => println!("Processed result: {}", result),
        Err(error) => eprintln!("Error processing boxed integer: {}", error),
    }

    // Call additional functions unrelated to the boxed integer resource
    other_actions_not_related_to_the_resource_1();
    other_actions_not_related_to_the_resource_2();
    other_actions_not_related_to_the_resource_3();
}


fn process_boxed_integer(b: &Box<i32>) -> Result<i32, &'static str> {
    let result = **b + 10;
    Ok(result)
}

fn other_actions_not_related_to_the_resource_1() {
    // ... 
}

fn other_actions_not_related_to_the_resource_2() {
    // ...
}

fn other_actions_not_related_to_the_resource_3() {
    // ...
}