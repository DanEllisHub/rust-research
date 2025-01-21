use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};
use ring::digest::{digest, SHA256};
use log::{info, warn, error, debug, trace, log};
use std::panic::panic_any;
use std::str;
use std::panic;
use std::error::Error;
use std::fmt;
use std::io::{stderr, Stderr};
use std::io::Write;

pub fn get_password(id: &str) -> String {
  //get password from db
  return "123-45-6789".to_string();
}

#[allow(unreachable_code)]
//vulnerable code
pub fn log_password(id: &str) -> String { 
  let password: String = get_password(&id);
  if true{
    panic!("error in password of user {id} : {password}", id = &id,password = &password); //sink
  }
  let error: String = format!("error in password of user {id} : {password}", id = &id,password = &password);
  panic_any(error);
  panic!("asd");
  return "logged password successfully!".to_string();
}


#[derive(Debug)]
struct InvalidPasswordError(String);

impl fmt::Display for InvalidPasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
//custom error trait that prints the invalid password (vulnerable)
impl Error for InvalidPasswordError {}

fn check_password(password: &str) -> Result<(), InvalidPasswordError> {
    if true {
        return Err(InvalidPasswordError(format!("Invalid password: {}", password)));//sink
    }
    Ok(())
}

//safe code
pub fn safe_log_password(id: &str) -> String { 
  let password: String = get_password(&id);
  if true{
    panic!("error in password of user {id}", id = &id);
  }
  let error: String = format!("error in password of user {id} : {password}", id = &id,password = &password);
  panic_any(error); //result  
  panic!("asd");
  return "logged password successfully!".to_string();
}


fn main() {
  let password: &str = "123456789";
  eprint!("(eprint) error with password: {}", password);//sink
  eprintln!("(eprintln) error with password: {}", password);//sink
  match check_password(password) {
    Ok(_) => {
        println!("password is valid");
    },
    Err(e) => {
      //let stderr: &mut Stderr  =  &mut std::io::stderr();
        println!("(println)An error occurred: {:?} while checking {}", e, password);//sink - printing password in case the result is Err
        print!("(print)An error occurred: {:?} while checking password {}", e, password);//sink - printing password in case the result is Err
        write!(&mut std::io::stderr(), "(write to stderr)An error occurred:  while checking password {}",password).unwrap();//sink - printing password in case the result is Err
    }
    }
}




