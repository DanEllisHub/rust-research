use data_encoding::HEXUPPER;

use ring::rand::SecureRandom;
use ring::{
    digest,  pbkdf2, rand
};
use std::num::NonZeroU32;

use ring::digest::SHA256_OUTPUT_LEN;
use ring::hkdf::Algorithm;

use ring::hkdf::Okm;
use ring::hkdf::Prk;
use ring::hkdf::Salt;
use ring::hkdf::HKDF_SHA256;

//vulnerable code
pub fn pbkdf2_weak() {
    const CREDENTIAL_LEN: usize = 20;
    let n_iter = NonZeroU32::new(999).unwrap(); //low iterations count
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; CREDENTIAL_LEN];
    //let salt = b"hardcodedsalt"; //hardcoded salt
    rng.fill(&mut salt).unwrap();
    let password = "Guess Me If You Can!";
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("Weak PBKDF: {}", HEXUPPER.encode(&pbkdf2_hash));
}

//vulnerable code
pub fn hkdf_weak() {
  let input_key_material = b"secret key";
  let salt = Salt::new(HKDF_SHA256, b"salt bytes"); //hardcoded salt
  let pseudo_rand_key: Prk = salt.extract(input_key_material);
  let context_data = &["context one".as_bytes()];
  let output_key_material: Okm<Algorithm> =
      pseudo_rand_key.expand(context_data, HKDF_SHA256).unwrap();
  let mut result = [0u8; SHA256_OUTPUT_LEN];
  output_key_material.fill(&mut result).unwrap();
  println!("WEAK HKDF: {}", HEXUPPER.encode(&result));
}

//safe code
pub fn pbkdf2() {
  const CREDENTIAL_LEN: usize = 20;
  let n_iter = NonZeroU32::new(600_000).unwrap();
  let rng = rand::SystemRandom::new();
  let mut salt = [0u8; CREDENTIAL_LEN];
  rng.fill(&mut salt).unwrap();
  let password = "Guess Me If You Can!";
  let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
  pbkdf2::derive(
      pbkdf2::PBKDF2_HMAC_SHA256,
      n_iter,
      &salt,
      password.as_bytes(),
      &mut pbkdf2_hash,
  );
  println!("Strong PBKDF: {}", HEXUPPER.encode(&pbkdf2_hash));
}
//safe code
pub fn hkdf() {
  const CREDENTIAL_LEN: usize = 20;
  let input_key_material = b"secret key";
  let rng = rand::SystemRandom::new();
  let mut salt_bytes = [0u8; CREDENTIAL_LEN];
  rng.fill(&mut salt_bytes).unwrap();
  let salt = Salt::new(HKDF_SHA256, &salt_bytes);
  let pseudo_rand_key: Prk = salt.extract(input_key_material);
  let context_data = &["context one".as_bytes()];
  let output_key_material: Okm<Algorithm> =
      pseudo_rand_key.expand(context_data, HKDF_SHA256).unwrap();
  let mut result = [0u8; SHA256_OUTPUT_LEN];
  output_key_material.fill(&mut result).unwrap();
  println!("Strong HKDF: {}", HEXUPPER.encode(&result));
}

fn main() {
    pbkdf2_weak();
    hkdf_weak();
    pbkdf2();
    hkdf();
}