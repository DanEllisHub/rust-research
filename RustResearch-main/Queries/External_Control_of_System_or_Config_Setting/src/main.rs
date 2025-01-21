use std::env;
use figment::Figment;
use figment::Profile;
use config::Config;
use config::builder::ConfigBuilder;
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    
    // std::env::set_var
    set_var("test_set_var");
    set_var_sanitized("test_set_var_sanitized");
    get_var();

    // Figment
    figment_from("test_from");
    figment_from_sanitized("test_from_sanitized");
    figment_join("test_join");
    figment_adjoin("test_adjoin");
    figment_merge("test_merge");
    figment_admerge("test_admerge");
    
    // Profile (Figment)
    profile_new("test_new");
    profile_new_sanitized("test_new_sanitized");
    profile_const_new("test_const_new");
    profile_from_env_or("test_from_env_or");

    // ConfigBuilder (Config)
    config_builder_set_default("test_set_default");
    config_builder_set_default_sanitized("test_set_default_sanitized");
    config_builder_set_override("test_set_override");
    config_builder_set_override_option("test_set_override_option");

}

// Example Rocket route with "host" parameter using std::env::set_var
// #[get("/setvar/<host>")]
fn set_var(host: &str) {
    let key = "key";
    env::set_var(key, host);    // Sink can be on both parameters
    println!("set_var: {}", env::var(key).unwrap());
}

// Example Rocket route with "host" parameter using std::env::set_var but input is sanitized by base64 encoding
// #[get("/setvar/sanitized/<host>")]
fn set_var_sanitized(host: &str) {
    let key = "key-sanitized";
    let sanitized = general_purpose::STANDARD.encode(host); // Sanitized
    env::set_var(key, sanitized);
    println!("set_var_sanitized: {}", env::var(key).unwrap());
}

// System Property Set in .env File
// Devs you can ignore this, this was mainly for DMS
// #[get("/getvar")]
fn get_var() {
    let key = "TESTDOTENV";
    let value = env::var(key).unwrap();
    println!("get_var_from_env: {}", env::var(key).unwrap());
}

// Example Rocket route with "host" parameter using Figment::from()
// #[get("/figment/from/<host>")]
fn figment_from(host: &str) {
    let figment = Figment::from(("key_from", host));   // Sink can be on both sides of the tuple
    println!("Figment: {:?}", figment);
}

// Example Rocket route with "host" parameter using Figment::from() but input is sanitized by base64 encoding
// #[get("/figment/from/sanitized/<host>")]
fn figment_from_sanitized(host: &str) {
    let sanitized = general_purpose::STANDARD.encode(host); // Sanitized
    let figment = Figment::from(("key_from_sanitized", sanitized));
    println!("Figment: {:?}", figment);
}

// Example Rocket route with "host" parameter using Figment::new().join
// #[get("/figment/join/<host>")]
fn figment_join(host: &str) {
    let figment = Figment::new().join(("key_join", host));   // Sink can be on both sides of the tuple
    println!("Figment: {:?}", figment);
}

// Example Rocket route with "host" parameter using Figment::new().adjoin
// #[get("/figment/adjoin/<host>")]
fn figment_adjoin(host: &str) {
    let figment = Figment::new().adjoin(("key_adjoin", host));   // Sink can be on both sides of the tuple
    println!("Figment: {:?}", figment);
}

// Example Rocket route with "host" parameter using Figment::new().merge
// #[get("/figment/merge/<host>")]
fn figment_merge(host: &str) {
    let figment = Figment::new().merge(("key_merge", host));   // Sink can be on both sides of the tuple
    println!("Figment: {:?}", figment);
}

// Example Rocket route with "host" parameter using Figment::new().admerge
// #[get("/figment/admerge/<host>")]
fn figment_admerge(host: &str) {
    let figment = Figment::new().admerge(("key_admerge", host));   // Sink can be on both sides of the tuple
    println!("Figment: {:?}", figment);
}

// Example Rocket route with "host" parameter using Profile::new
// #[get("/figment/profile/new/<host>")]
fn profile_new(host: &str) {
    let profile = Profile::new( host);   // Sink
    println!("Profile: {:?}", profile);
}

// Example Rocket route with "host" parameter using Profile::new but input is sanitized by base64 encoding
// #[get("/figment/profile/new/sanitized/<host>")]
fn profile_new_sanitized(host: &str) {
    let sanitized = general_purpose::STANDARD.encode(host); // Sanitized
    let profile = Profile::new(&sanitized);
    println!("Profile: {:?}", profile);
}

// Example Rocket route with "host" parameter using Profile::const_new
// #[get("/figment/profile/const_new/<host>")]
fn profile_const_new(host: &'static str) {
    let profile = Profile::const_new(host);   // Sink
    println!("Profile: {:?}", profile);
}

// Example Rocket route with "host" parameter using Profile::from_env_or
// #[get("/figment/profile/from_env_or/<host>")]
fn profile_from_env_or(host: &str) {
    let profile = Profile::from_env_or("key_from_env_or", host);   // Sink on second parameter
    println!("Profile: {:?}", profile);
}

// Example Rocket route with "host" parameter using ConfigBuilder::set_default
// #[get("/config/builder/set_default/<host>")]
fn config_builder_set_default(host: &str) {
    let builder = Config::builder();
    let config = builder.set_default("key_set_default", host).unwrap().build().unwrap();   // Sink can be on both parameters
    println!("ConfigBuilder: {:?}", config.get::<String>("key_set_default").ok());
}

// Example Rocket route with "host" parameter using ConfigBuilder::set_default but input sanitized by base64 encoding
// #[get("/config/builder/set_default/sanitized/<host>")]
fn config_builder_set_default_sanitized(host: &str) {
    let builder = Config::builder();
    let sanitized = general_purpose::STANDARD.encode(host);  // Sanitized
    let config = builder.set_default("key_set_default_sanitized", sanitized).unwrap().build().unwrap();
    println!("ConfigBuilder: {:?}", config.get::<String>("key_set_default_sanitized").ok());
}

// Example Rocket route with "host" parameter using ConfigBuilder::set_override
// #[get("/config/builder/set_override/<host>")]
fn config_builder_set_override(host: &str) {
    let builder = Config::builder();
    let config = builder.set_override("key_set_override", host).unwrap().build().unwrap();   // Sink can be on both parameters
    println!("ConfigBuilder: {:?}", config.get::<String>("key_set_override").ok());
}

// Example Rocket route with "host" parameter using ConfigBuilder::set_override_option
// #[get("/config/builder/set_override_options/<host>")]
fn config_builder_set_override_option(host: &str) {
    let builder = Config::builder();
    let config = builder.set_override_option("key_set_override_options", Some(host)).unwrap().build().unwrap();   // Sink can be on both parameters
    println!("ConfigBuilder: {:?}", config.get::<String>("key_set_override_options").ok());
}