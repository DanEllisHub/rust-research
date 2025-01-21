
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};
use std::io::Bytes;
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;
use rand;
use rand::RngCore;
use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;


const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
//vulnerable code
  pub fn random(password: &str) ->  Vec<0u8> { 
    let mut salt = [0u8; CREDENTIAL_LEN];
    let mut r = rand::thread_rng(); //unsafe RNG
    r.fill_bytes(&mut salt);
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt, //Sink
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    return pbkdf2_hash.to_vec()
  }


  
//safe code
  pub fn random_safe(password: &str) -> Vec<0u8> {
    let mut salt = [0u8; CREDENTIAL_LEN];
    let mut r = ring::rand::SystemRandom::new(); //safe RNG
    r.fill_bytes(&mut salt);
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    return pbkdf2_hash.to_vec()
  }

  fn main(){
    println!(random("asdasdasd"));
    println!(random_safe("asdasdasd"));
  }


