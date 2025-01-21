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

pub fn get_ssn(id: &str) -> String {
  //get ssn from db
  return "123-45-6789".to_string();
}

#[allow(unreachable_code)]
//vulnerable code
pub fn log_ssn(id: &str) -> String { 
  let ssn: String = get_ssn(&id); //PII
  if true{
    panic!("error in ssn of user {id} : {ssn}", id = &id,ssn = &ssn); //sink
  }
  let error: String = format!("error in ssn of user {id} : {ssn}", id = &id,ssn = &ssn);
  panic_any(error); //result  
  panic!("asd");
  return "logged ssn successfully!".to_string();
}

//custom error trait that prints the invalid ssn
#[derive(Debug)]
struct InvalidSsnError(String);

impl fmt::Display for InvalidSsnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for InvalidSsnError {}

fn check_ssn(ssn: &str) -> Result<(), InvalidSsnError> {
    if true {
        return Err(InvalidSsnError(format!("Invalid SSN: {}", ssn)));//sink
    }
    Ok(())
}


fn main() {
  let ssn: &str = "123456789";
  eprint!("(eprint) error with SSN: {}", ssn);//sink
  eprintln!("(eprintln) error with SSN: {}", ssn);//sink
  match check_ssn(ssn) {
    Ok(_) => {
        println!("SSN is valid");
    },
    Err(e) => {
      //let stderr: &mut Stderr  =  &mut std::io::stderr();
        println!("(println)An error occurred: {:?} while checking {}", e, ssn);//sink - printing PII in case the result is Err
        print!("(print)An error occurred: {:?} while checking ssn {}", e, ssn);//sink - printing PII in case the result is Err
        write!(&mut std::io::stderr(), "(write to stderr)An error occurred:  while checking ssn {}",ssn).unwrap();//sink - printing PII in case the result is Err
    }
    }
}




