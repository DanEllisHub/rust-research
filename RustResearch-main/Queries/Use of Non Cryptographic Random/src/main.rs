
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};
use std::io::Bytes;
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;
use rand;
use rand::RngCore;

//vulnerable code
  pub fn random() ->  Vec<u8> { 
    let mut data = [0u8; 8];
    let mut r = rand::thread_rng(); //Sink
    r.fill_bytes(&mut data);
    println!("rand random bytes{:?}", data);
    return data.to_vec();
  }

  
//safe code
  pub fn random_safe() -> [u8; 8] {
    let mut randoms: [u8; 8] = [0; 8];
    let sr = ring::rand::SystemRandom::new();
    sr.fill(&mut randoms);
    println!("ring random bytes: {:?}", randoms);
    return randoms;
  }
  fn main(){
    random();
    random_safe();
  }


