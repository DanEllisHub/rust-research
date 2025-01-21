#[macro_use] extern crate rocket;
use serde::{Deserialize, Serialize};
use std::io::Bytes;
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;
use data_encoding::{HEXLOWER, HEXUPPER};
use md4::Md4;
use md4::Digest;
use sha1::Sha1;
use ring::{
    digest,
    hkdf::{Algorithm, Okm, Prk, Salt, HKDF_SHA1_FOR_LEGACY_USE_ONLY},
    hmac, rand, signature,
};
use std::str;


//vulnerable code
#[get("/?<data>")]
fn index(data: String) -> String {
  let mut bytes = [0u8; 32];
  let rng = SystemRandom::new();
  rng.fill(&mut bytes).unwrap();
  let data_with_salt = format!("{:?}{:?}",&data, String::from_utf8_lossy(&bytes).to_string()); //user data with salt
  let digest = md5::compute(&data_with_salt.as_bytes()); // SINK
  let mut hasher = Md4::new();
  hasher.update(&data_with_salt.as_bytes()); // SINK
  let result = hasher.finalize();
  let mut hasher = Sha1::new();
  hasher.update(&data_with_salt.as_bytes()); // SINK
  let mut ctx = digest::Context::new(&digest::SHA256);
  ctx.update(&data_with_salt.as_bytes()); // SINK
  let digested = digest::digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, &data.as_bytes()); //SINK
  return HEXUPPER.encode(&digest.as_ref());
}

//safe code
#[get("/safe?<data1>&<data2>")]
fn index1(data1: String, data2: String) -> String {
  let conc_data = data1.clone() + &data2;
  let digest = md5::compute(&conc_data.as_bytes()); //FP - no secret key or salt concatenation
  let mut bytes = [0u8; 32];
  let rng = rand::SystemRandom::new();
  let key = hmac::Key::generate(hmac::HMAC_SHA256, &rng).unwrap();
  let tag = hmac::sign(&key, data1.as_bytes()); //FP - hmac is safe from length extension
  return HEXUPPER.encode(&digest.as_ref());
}  



//create a rocket launcher and a route
#[launch]
fn rocket() -> _ {
  rocket::build()
      .mount("/", routes![index])
      .mount("/safe", routes![index1])
}

