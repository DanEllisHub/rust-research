use aes_gcm::aes;
use data_encoding::HEXUPPER;
use ring::aead::UnboundKey;
use ring::rand;
use ring::digest::{SHA256_OUTPUT_LEN, SHA512_OUTPUT_LEN};
use ring::{hkdf::{Salt, HKDF_SHA256, Prk}};
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::pbkdf2;
use std::{num::NonZeroU32, error::Error};
use ring::aead::AES_128_GCM;
use ring::aead::Aad;
use ring::aead::AES_256_GCM;
use ring::aead::BoundKey;
use ring::aead::SealingKey;
use ring::aead::OpeningKey;
use ring::aead::NonceSequence;
use ring::aead::NONCE_LEN;
use ring::rand::SystemRandom;
use ring::aead::Nonce;
use ring::hkdf::HKDF_SHA384;
use ring::hkdf::HKDF_SHA512;
use ring::hkdf::Algorithm;
use ring::hkdf::Okm;

use ring::signature::Ed25519KeyPair;
use ring::signature::ECDSA_P256_SHA256_ASN1_SIGNING;

pub fn hardcoded() -> Result<(), Unspecified> {
    //Hardcoded key in HKDF
    let rand = rand::SystemRandom::new();
    // Generate a new symmetric encryption key
    let mut secret_key = vec![0; AES_256_GCM.key_len()];
    //key_bytes = HEXUPPER.decode("D45EFE723667F9CCB3D60AB60119302D4581C53CBB58C3058E4FF834A1A6A59A")

    rand.fill(&mut secret_key)?;
    //let secret_key = b"secret key";
    let salt = Salt::new(HKDF_SHA256, b"salt as bytes");
    let pseudo_rand_key: Prk = salt.extract(&secret_key);

    let context_data = &["context one".as_bytes()];
    let output_key_material = pseudo_rand_key.expand(context_data, HKDF_SHA256).unwrap();
    let mut result = [0u8; SHA256_OUTPUT_LEN];
    output_key_material.fill(&mut result).unwrap();
    println!("Derived output key material: {}", HEXUPPER.encode(&result));

    //Hardcoded key in AEAD
    let rand = rand::SystemRandom::new();
    // Generate a new symmetric encryption key
    let mut key_bytes = vec![0; AES_256_GCM.key_len()];
    //key_bytes = HEXUPPER.decode("D45EFE723667F9CCB3D60AB60119302D4581C53CBB58C3058E4FF834A1A6A59A")

    rand.fill(&mut key_bytes)?;
    println!("AAAAAAAAAAA {}", HEXUPPER.encode(&key_bytes));
    println!("key_bytes = {}", HEXUPPER.encode(&key_bytes)); 
    // Create a new AEAD key without a designated role or nonce sequence
    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)?;

    struct CounterNonceSequence(u32);
    impl NonceSequence for CounterNonceSequence {
        // called once for each seal operation
        fn advance(&mut self) -> Result<Nonce, Unspecified> {
            let mut nonce_bytes = vec![0; NONCE_LEN];
            let bytes = self.0.to_be_bytes();
            nonce_bytes[8..].copy_from_slice(&bytes);
            println!("nonce_bytes = {}", HEXUPPER.encode(&nonce_bytes));
            self.0 += 1; // advance the counter
            Nonce::try_assume_unique_for_key(&nonce_bytes)
        }
    }
    // Create a new NonceSequence type which generates nonces
    let nonce_sequence = CounterNonceSequence(1);
    // Create a new AEAD key for encrypting and signing ("sealing"), bound to a nonce sequence
    // The SealingKey can be used multiple times, each time a new nonce will be used
    let mut sealing_key = SealingKey::new(unbound_key, nonce_sequence);
    // This data will be authenticated but not encrypted
    //let associated_data = Aad::empty(); // is optional so can be empty
    let associated_data = Aad::from(b"additional public data");
    // Data to be encrypted
    let data = b"hello world";
    println!("data = {}", String::from_utf8(data.to_vec()).unwrap());
    // Create a mutable copy of the data that will be encrypted in place
    let mut in_out = data.clone();
    // Encrypt the data with AEAD using the AES_256_GCM algorithm
    let tag = sealing_key.seal_in_place_separate_tag(associated_data, &mut in_out)?; //Encryption Sink
    println!("encrypted data: {}", HEXUPPER.encode(&in_out));


    const CREDENTIAL_LEN: usize = SHA512_OUTPUT_LEN;
    let n_iter: NonZeroU32 = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();
    let mut salt: [u8; 64] = [0u8; CREDENTIAL_LEN];
    let password = "Guess Me If You Can!";
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    return Ok(());

}