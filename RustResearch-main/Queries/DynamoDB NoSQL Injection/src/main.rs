#[macro_use]
extern crate rocket;
use aws_sdk_dynamodb;
use aws_sdk_dynamodb::types::BatchStatementRequest;
use aws_sdk_lambda;
use aws_sdk_s3 as s3;
use aws_sdk_s3control as s3control;
use rocket::form;
use s3::client;
use s3::primitives::ByteStream;
use std::env;
use std::fs;

// bucket = "ori-public-bucket-rust";
// key = "test.txt";

static PATH: &str = "../../file-body.txt";
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};

#[get("/db?<input>")]
async fn db(input: String) -> String {
    let client = configure_dynamo_db_client().await;
    let stored_input = get_stored_input();
    execute_partiql_query_1(client.clone(), input.as_str()).await; // vulnerable - this flow will lead to a result
    execute_partiql_query_2(client.clone(), stored_input.as_str()).await; // vulnerable - this flow will lead to a result


    execute_partiql_query_3(client.clone(), input.as_str()).await; // safe - this is sanitized with whitelist
    "aaa".to_string()
}

fn get_stored_input() -> String {
    let path = "stored.txt".to_string();
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    return contents;
}

async fn execute_partiql_query_1(client: aws_sdk_dynamodb::Client, input_query: &str) {
    let query = format!(
        "INSERT INTO \"rust_dynamodb_nosql_injection_query\" VALUE {}",
        input_query
    );

    let response = client
        .execute_statement()
        .statement(query) // RESULT - NoSQL Injection
        .send()
        .await
        .unwrap();
    println!("Response: {:?}", response);
}

async fn execute_partiql_query_2(client: aws_sdk_dynamodb::Client, input_query: &str) {
    let response = client
        .execute_statement()
        .statement(input_query.to_string()) // RESULT - NoSQL Injection
        .send()
        .await
        .unwrap();
    println!("Response: {:?}", response);
}

async fn execute_partiql_query_3(client: aws_sdk_dynamodb::Client, input_query: &str) {

    let input_query = whitelist(input_query); // SANITIZER

    let query = format!(
        "INSERT INTO \"rust_dynamodb_nosql_injection_query\" VALUE {}",
        input_query
    );

    let response = client
        .execute_statement()
        .statement(query) // SAFE - this is sanitized by whitelist
        .send()
        .await
        .unwrap();
    println!("Response: {:?}", response);
}

fn whitelist(input: &str) -> String {
    if input == "something" {
        return "something".to_string();
    } else {
        return "default".to_string();
    }
}

// configure dynamo-db client
async fn configure_dynamo_db_client() -> aws_sdk_dynamodb::Client {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    return client;
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![db])
}
