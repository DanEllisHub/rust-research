
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};
use std::io::Bytes;
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;
use ring::{digest, test};
use rand;
use rand::RngCore;
use subtle::*;

//vulnerable code
  pub fn compare_hash(a: &Vec<u8>, b: &Vec<u8>) ->  bool { 
    if &a == &b {
      return true;
    }
    return false;
  }

  
//safe code
pub fn compare_hash_ct(a: &Vec<u8>, b: &Vec<u8>) ->  bool { 
  if (&a).ct_eq(&b).unwrap_u8() == 1 {
    return true;
  }
  return false;
}
  fn main(){
    let a = format!("{:?}", digest::digest(&digest::SHA256, b"hello, world")).into_bytes();
    let b = format!("{:?}", digest::digest(&digest::SHA256, b"hello, worldddd")).into_bytes();
    println!("{:?}", a);
    println!("{}", compare_hash(&a, &b).to_string());
    println!("{}", compare_hash_ct(&a, &b).to_string());
  }


