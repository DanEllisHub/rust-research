#[macro_use] extern crate rocket;
pub mod models;
pub mod schema;
use diesel::Connection;
use diesel::connection::DefaultLoadingMode;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sql_query;
use diesel::sql_types::BigInt;
use diesel::sql_types::Integer;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use schema::posts;
use diesel::prelude::*;
use self::models::{NewPost, Post};
use std::env;
use data_encoding::HEXUPPER;
use ring::hkdf::HKDF_SHA256;
use ring::rand::SecureRandom;
use ring::{
    digest,  pbkdf2, rand
};
use std::num::NonZeroU32;
use ring::hkdf::Salt;
use ring::digest::SHA256_OUTPUT_LEN;



pub fn connect_diesel() -> PgConnection {
    let database_url = "postgres://postgres:postgres@localhost/diesel_db";
    diesel::PgConnection::establish(&database_url).expect("msg")
}


pub fn save_password(user: &str, password: &str, conn: &mut PgConnection) {
    let check = sql_query("UPDATE users SET password = $2 WHERE user = $1")
        .bind::<diesel::sql_types::Text, _>(&password)
        .bind::<diesel::sql_types::Text, _>(&user)
        .execute(conn);
}
//vulnerable code
#[get("/?<user>&<password>")]
fn change_password(user: String, password: String) -> String {
    let mut conn = connect_diesel();
    save_password(&user, &password, &mut conn);
    format!("New password for user {:?} saved: ", user)
}

//safe code
const CREDENTIAL_LEN: usize = 64;

fn change_password_encrypted(user: String, password: String) -> String {
    let mut conn = connect_diesel();
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt).unwrap();
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    let pass_digest: String = HEXUPPER.encode(&pbkdf2_hash);
    save_password(&user, &pass_digest, &mut conn);
    format!("New password for user {:?} saved: ", user)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![change_password])
        .mount("/", routes![change_password_encrypted])
}

