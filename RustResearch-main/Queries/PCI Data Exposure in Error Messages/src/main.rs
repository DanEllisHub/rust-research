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

pub fn get_pan(id: &str) -> String {
  //get pan from db
  return "123-45-6789".to_string();
}

#[allow(unreachable_code)]
//vulnerable code
pub fn log_pan(id: &str) -> String { 
  let pan: String = get_pan(&id); //PII
  if true{
    panic!("error in pan of user {id} : {pan}", id = &id,pan = &pan); //sink
  }
  let error: String = format!("error in pan of user {id} : {pan}", id = &id,pan = &pan);
  panic_any(error); //result  
  panic!("asd");
  return "logged pan successfully!".to_string();
}

//custom error trait that prints the invalid pan
#[derive(Debug)]
struct InvalidpanError(String);

impl fmt::Display for InvalidpanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for InvalidpanError {}

fn check_pan(pan: &str) -> Result<(), InvalidpanError> {
    if true {
        return Err(InvalidpanError(format!("Invalid pan: {}", pan)));//sink
    }
    Ok(())
}


fn main() {
  let pan: &str = "123456789";
  eprint!("(eprint) error with pan: {}", pan);//sink
  eprintln!("(eprintln) error with pan: {}", pan);//sink
  match check_pan(pan) {
    Ok(_) => {
        println!("pan is valid");
    },
    Err(e) => {
      //let stderr: &mut Stderr  =  &mut std::io::stderr();
        println!("(println)An error occurred: {:?} while checking {}", e, pan);//sink - printing PII in case the result is Err
        print!("(print)An error occurred: {:?} while checking pan {}", e, pan);//sink - printing PII in case the result is Err
        write!(&mut std::io::stderr(), "(write to stderr)An error occurred:  while checking pan {}",pan).unwrap();//sink - printing PII in case the result is Err
    }
    }
}




