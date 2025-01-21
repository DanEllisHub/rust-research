// https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html
#![allow(unconditional_panic)]
#![allow(dead_code)]
// Suppress all warnings
#![allow(warnings)]

use rand::Rng;
use std::io;
use zeroize::{Zeroize, ZeroizeOnDrop};

// This struct will be zeroized on drop - sanitizer
#[derive(ZeroizeOnDrop)]
struct User {
    username: String,
    email: String,
    password: String,
}

// This struct won't be zeroized on drop
struct UnsafeUser {
    username: String,
    email: String,
    password: String,
}

fn main() {
    sample_1();
    sample_2();
    secure_sample_1();
    secure_sample_2();
    secure_sample_3();
    secure_sample_4();
}

// Using user's secret
fn sample_1() {
    // Ask the user for a password
    let mut secret = get_user_input("Please enter a password: ");   // a result ends here - sink
    
    // bla bla bla use the password bla bla bla

    unsafe {println!("password starts with '{:?}'", secret.chars().next());}
}

// Using implicit drop won't erase the data
fn sample_2() {
    let mut input = get_user_input("Please enter a password: ");
    let ptr : *const UnsafeUser;

    {   // Opening a scope to use user1
        let user1 = UnsafeUser {
            username: String::from("someusername123"),
            password: input,
            email: String::from("someone@example.com"),
        };

        ptr = &user1;

        unsafe {
            println!("password starts with {:?}", (*ptr).password.chars().next());
        }

    }   // Closing the scope to drop user1

    unsafe {
        // implicit drop() does not clear the password field of user1
        println!("password starts with {:?}", (*ptr).password.chars().next());
    }
}

// Using zeroize() function
fn secure_sample_1() {
    // Ask the user for a password
    let mut secret = get_user_input("Please enter a password: ");
    
    // bla bla bla use the password bla bla bla
    unsafe {println!("password starts with '{:?}'", secret.chars().next());}

    // Now that we're done using the secret, zero it out - sanitizer
    secret.zeroize();

    unsafe {println!("password starts with '{:?}'", secret.chars().next());}
}

// Using ZeroizeOnDrop trait on implicit drop
fn secure_sample_2() {
    let mut input = get_user_input("Please enter a password: ");
    let ptr : *const String;

    {   // Opening a scope to use user1
        let user1 = User {
            username: String::from("someusername123"),
            password: input,
            email: String::from("someone@example.com"),
        };

        ptr = &user1.password;

        unsafe {
            // input is 'P' and user1 is 'P'
            println!("password starts witt {:?}", (*ptr).chars().next());
        }

    }   // Closing the scope to drop user1

    unsafe {
        // input is '\u{efae}' and user1 is None
        // implicit drop() zeroized the password field of user1
        println!("password starts with {:?}", (*ptr).chars().next());
    }
}

// ZeroizeOnDrop trait on implicit drop
fn secure_sample_3() {
    let mut input = get_user_input("Please enter a password: ");
    let ptr : *const String;

    let user1 = User {
        username: String::from("someusername123"),
        password: input,
        email: String::from("someone@example.com"),
    };

    ptr = &user1.password;

    unsafe {
        // input is 'P' and user1 is 'P'
        println!("password starts witt {:?}", (*ptr).chars().next());
    }

    // Ending the scope will implicitly drop the user method
}

// Not using secrets
fn secure_sample_4() {
    // Ask the user for a password
    let mut username = get_user_input("Please enter a username: ");   // username is not a secret
    
    // bla bla bla use the password bla bla bla

    unsafe {println!("username starts with '{:?}'", username.chars().next());}
}

// Get user input
fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    // Create a mutable string to store the user input
    let mut input = String::new();

    // Read input from the console
    io::stdin().read_line(&mut input)   // Input - results start here
        .expect("Failed to read line");

    // Remove trailing newline character
    input.trim().to_string()
}