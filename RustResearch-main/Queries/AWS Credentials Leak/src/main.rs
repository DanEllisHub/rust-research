#[macro_use]
extern crate rocket;
use aws_sdk_s3 as s3;
use aws_sdk_lambda;
use aws_sdk_dynamodb;
use aws_sdk_s3control as s3control;
use rocket::form;
use s3::primitives::ByteStream;
use std::fs;
use std::env;

// bucket = "ori-public-bucket-rust";
// key = "test.txt";

static PATH: &str = "../../file-body.txt";

#[get("/init_creds")]
async fn init_creds() -> String {
    // TOTAL RESULTS: 18

    let lambda_creds: aws_sdk_lambda::config::Credentials = get_lambda_creds();
    let s3_creds: aws_sdk_s3::config::Credentials = get_s3_creds();
    let dynamo_creds: aws_sdk_dynamodb::config::Credentials = get_dynamo_creds();

    println!("AWS Lambda Credentials: {:?}", lambda_creds.access_key_id()); // RESULT - print to console
    println!("AWS Lambda Credentials: {:?}", lambda_creds.secret_access_key()); // RESULT - print to console
    println!("AWS Lambda Credentials: {:?}", lambda_creds.session_token()); // RESULT - print to console

    println!("AWS Lambda Credentials: {:?}", s3_creds.access_key_id()); // RESULT - print to console
    println!("AWS Lambda Credentials: {:?}", s3_creds.secret_access_key()); // RESULT - print to console
    println!("AWS Lambda Credentials: {:?}", s3_creds.session_token()); // RESULT - print to console

    println!("AWS Lambda Credentials: {:?}", dynamo_creds.access_key_id()); // RESULT - print to console
    println!("AWS Lambda Credentials: {:?}", dynamo_creds.secret_access_key()); // RESULT - print to console
    println!("AWS Lambda Credentials: {:?}", dynamo_creds.session_token()); // RESULT - print to console


    let response = 
        format!(
            "AWS Lambda Credentials: {:?}, {:?}, {:?}, AWS S3 Credentials: {:?}, {:?}, {:?}, AWS DynamoDB Credentials: {:?}, {:?}, {:?}",
            lambda_creds.access_key_id(), lambda_creds.secret_access_key(), lambda_creds.session_token(),
            s3_creds.access_key_id(), s3_creds.secret_access_key(), s3_creds.session_token(),
            dynamo_creds.access_key_id(), dynamo_creds.secret_access_key(), dynamo_creds.session_token()
        );
    
    response // More 9 RESULTS - interactive output of Rocket
}

fn get_lambda_creds() -> aws_sdk_lambda::config::Credentials {
    aws_sdk_lambda::config::Credentials::new(
        "hardcoded_key_id".to_string(),
        "hardcoded_key_secret".to_string(),
        Some("hardcoded_session_token".to_string()),
        None,
        "loaded-from-custom-env"
    )
}

fn get_s3_creds() -> aws_sdk_s3::config::Credentials {
    aws_sdk_s3::config::Credentials::new(
        "hardcoded_key_id".to_string(),
        "hardcoded_key_secret".to_string(),
        Some("hardcoded_session_token".to_string()),
        None,
        "loaded-from-custom-env"
    )
}

fn get_dynamo_creds() -> aws_sdk_dynamodb::config::Credentials {
    aws_sdk_dynamodb::config::Credentials::new(
        "hardcoded_key_id".to_string(),
        "hardcoded_key_secret".to_string(),
        Some("hardcoded_session_token".to_string()),
        None,
        "loaded-from-custom-env"
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![init_creds])
}
