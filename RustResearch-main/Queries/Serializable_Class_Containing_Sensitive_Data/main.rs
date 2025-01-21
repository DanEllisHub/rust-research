#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String, // Result
}

#[derive(Serialize, Deserialize, Debug)]
struct UserFP {
    username: String,
    //also #[serde(skip)]
    #[serde(skip_serializing,skip_deserializing)]
    password: String, // No result
}

fn main() {
    let user = User {username: String::from("Test"), password: String::from("SuperSecret!@#!@#")};

    let serialized = serde_json::to_string(&user).unwrap();

    println!("unsecure serialized = {}", serialized);

    let deserialized: User = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);

    let userfp = UserFP {username: String::from("Test"), password: String::from("SuperSecret!@#!@#")};

    let serialized = serde_json::to_string(&userfp).unwrap();

    println!("secure serialized = {}", serialized);

    let deserialized: UserFP = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);
}
