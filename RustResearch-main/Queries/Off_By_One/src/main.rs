use std::panic;

fn main() {
    println!("Hello, world!");
    let mut array: [i32; 4] = [0; 4];
    array[1] = 1;
    array[2] = 2;
    array[3] = 3;
    println!("Length: {}", array.len());

    // Simplest solution - foreach-like iteration
    for x in array {
        println!("{x}");
    }

    // Correct len
    for x in 0..array.len() {
        println!("{x} {}", array[x]);
    }


    // Will panic
    let mut result = panic::catch_unwind(|| {
        for x in 0..array.len()+1 {
            println!("{x} {}", array[x]); // panics at x=4 - RESULT
        }
    }); 
    println!("array.len()+1 error {}", result.is_err());


    for x in 0..array.len()+1 {
        if (x < array.len()) { // prevents panic
            println!("{x} {}", array[x]); // Size check sanitizes result
        }
    }

    // Will panic
    let mut result = panic::catch_unwind(|| {
        for x in 0..array.len() {
            println!("{x} {}", array[x+1]); // panics at x=3 - RESULT
        }
    }); 
    println!("array.len()+1 error {}", result.is_err());   

    // Will panic
    result = panic::catch_unwind(|| {
        for x in (0..array.len()+1).step_by(2) {
            println!("{x} {}", array[x]); // panics at x=4 - RESULT
        }
    }); 
    println!("{}", result.is_err());

    // Won't panic because array.len happens to not exceed range with step_by, but if array is dynamic it might
    for x in (0..array.len()+1).step_by(3) {
        println!("{x} {}", array[x]); // Result despite no panic - this would be impossible to discern with dynamic arrays and code is still weak
    }
}
