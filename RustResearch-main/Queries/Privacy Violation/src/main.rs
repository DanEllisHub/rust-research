#[macro_use] extern crate rocket;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};

//vulnerable code
  #[get("/ssn?<ssn>&<id>")]
  fn changeSSN(name: String, ssn: String) -> String { 
      saveSSN(&id, &ssn);
      format!("new SSN: {} ", &ssn)
  }

  pub fn saveSSN(id: &String, ssn: &String) {
    //save id to db
  }


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/ssn", routes![changeSSN])
}