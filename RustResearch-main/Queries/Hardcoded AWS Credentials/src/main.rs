#[macro_use]
extern crate rocket;
use aws_sdk_s3 as s3;
use aws_sdk_lambda;
use aws_sdk_dynamodb;
use aws_sdk_s3control as s3control;
use s3::primitives::ByteStream;
use std::fs;
use std::env;

// bucket = "ori-public-bucket-rust";
// key = "test.txt";

static PATH: &str = "../../file-body.txt";
static HARDCODED_GLOBAL_CONSTANT: &str = "SOME-HARDCODED-CONSTANT";

#[get("/init_creds")]
async fn init_creds() -> String {
    let lambda_creds: aws_sdk_lambda::config::Credentials = get_lambda_creds(); // 3 RESULTS
    let s3_creds: aws_sdk_s3::config::Credentials = get_s3_creds(); // 3 RESULTS
    let dynamo_creds: aws_sdk_dynamodb::config::Credentials = get_dynamo_creds(); // 3 RESULTS

    let lambda_creds_safe: aws_sdk_lambda::config::Credentials = get_lambda_creds_safe(); // SAFE
    let s3_creds_safe: aws_sdk_s3::config::Credentials = get_s3_creds_safe(); // SAFE
    let dynamo_creds_safe: aws_sdk_dynamodb::config::Credentials = get_dynamo_creds_safe(); // SAFE

    "".to_string()
}

fn get_hardcoded_string() -> String {
    "hardcoded_string".to_string()
}

fn get_lambda_creds() -> aws_sdk_lambda::config::Credentials {
    let hardcoded_from_function = get_hardcoded_string();
    aws_sdk_lambda::config::Credentials::new(
        hardcoded_from_function, // RESULT - sink with hardcoded string
        "hardcoded_key_secret".to_string(), // RESULT - sink with hardcoded string
        Some("hardcoded_session_token".to_string()), // RESULT - sink with hardcoded string
        None, // NOT A SINK
        "loaded-from-custom-env" // NOT A SINK
    )
}

fn get_s3_creds() -> aws_sdk_s3::config::Credentials {
    aws_sdk_s3::config::Credentials::new(
        HARDCODED_GLOBAL_CONSTANT.to_string(), // RESULT - sink with hardcoded string
        "hardcoded_key_secret".to_string(), // RESULT - sink with hardcoded string
        Some("hardcoded_session_token".to_string()), // RESULT - sink with hardcoded string
        None, // NOT A SINK
        "loaded-from-custom-env" // NOT A SINK
    )
}

fn get_dynamo_creds() -> aws_sdk_dynamodb::config::Credentials {
    aws_sdk_dynamodb::config::Credentials::new(
        "hardcoded_key_id".to_string(), // RESULT - sink with hardcoded string
        "hardcoded_key_secret".to_string(), // RESULT - sink with hardcoded string
        Some("hardcoded_session_token".to_string()), // RESULT - sink with hardcoded string
        None, // NOT A SINK
        "loaded-from-custom-env" // NOT A SINK
    )
}



fn get_lambda_creds_safe() -> aws_sdk_lambda::config::Credentials {
    let aws_access_key_id = env::var("aws_access_key_id").unwrap();
    let aws_secret_access_key = env::var("aws_secret_access_key").unwrap();
    let aws_session_token = env::var("aws_session_token").unwrap();

    aws_sdk_lambda::config::Credentials::new(
        aws_access_key_id, // SAFE - the value is NOT hardcoded
        aws_secret_access_key, // SAFE - the value is NOT hardcoded
        Some(aws_session_token), // SAFE - the value is NOT hardcoded
        None, // NOT A SINK
        "loaded-from-custom-env" // NOT A SINK
    )
}

fn get_s3_creds_safe() -> aws_sdk_s3::config::Credentials {
    let aws_access_key_id = env::var("aws_access_key_id").unwrap();
    let aws_secret_access_key = env::var("aws_secret_access_key").unwrap();
    let aws_session_token = env::var("aws_session_token").unwrap();

    aws_sdk_s3::config::Credentials::new(
        aws_access_key_id, // SAFE - the value is NOT hardcoded
        aws_secret_access_key, // SAFE - the value is NOT hardcoded
        Some(aws_session_token), // SAFE - the value is NOT hardcoded
        None, // NOT A SINK
        "loaded-from-custom-env" // NOT A SINK
    )
}

fn get_dynamo_creds_safe() -> aws_sdk_dynamodb::config::Credentials {
    let aws_access_key_id = env::var("aws_access_key_id").unwrap();
    let aws_secret_access_key = env::var("aws_secret_access_key").unwrap();
    let aws_session_token = env::var("aws_session_token").unwrap();

    aws_sdk_dynamodb::config::Credentials::new(
        aws_access_key_id, // SAFE - the value is NOT hardcoded
        aws_secret_access_key, // SAFE - the value is NOT hardcoded
        Some(aws_session_token), // SAFE - the value is NOT hardcoded
        None, // NOT A SINK
        "loaded-from-custom-env" // NOT A SINK
    )
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![init_creds])
}
