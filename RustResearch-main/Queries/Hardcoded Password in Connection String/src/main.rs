use std::fmt::format;

pub mod models;
pub mod schema;

//let database_url = "postgres://postgres:postgres@localhost/diesel_db";

// TODO:
// ENABLE THE "SYNC" FEATURE IN MONGO CARGO.TOML (REFER TO THE RESEARCH FOR FURTHER INFO)
pub fn sync_mongo_uri() -> Result<mongodb::sync::Client, mongodb::error::Error> {
    use mongodb::{
        bson::{doc, extjson::de::Error as MongoError, oid::ObjectId, Document},
        results::{InsertOneResult, UpdateResult},
        sync::{Client, Collection},
    };

    let password = "HARDCODED_PASSWORD";
    let mongo_uri_hardcoded_password = format!("mongodb://USERNAME:{}@localhost:27017/?authMechanism=DEFAULT", password);
    //let mongo_uri_empty_password = "mongodb://USERNAME@localhost:27017/?authMechanism=DEFAULT";

    //let client = Client::with_uri_str("mongodb://localhost:27017"); // RESULT - empty password
    let client2 = Client::with_uri_str("mongodb://USERNAME:HARDCODED_PASSWORD@localhost:27017/?authMechanism=DEFAULT"); // RESULT - hardcoded password
    let client3 = Client::with_uri_str(mongo_uri_hardcoded_password); // RESULT
    //let client4 = Client::with_uri_str(mongo_uri_empty_password); // RESULT
    
    client3
}

pub async fn async_mongo_uri() -> Result<mongodb::Client, mongodb::error::Error> {
    // FOR THIS CODE TO RUN
    // THE CARGO.toml "tokio-sync" feature should be turned off

    use mongodb::{options::ClientOptions, Client, };

    // Parse a connection string into an options struct.
    let mut client_options: mongodb::options::ClientOptions =
        mongodb::options::ClientOptions::parse("mongodb://user:HARDCODED_PASSWORD@localhost:27017").await.expect("msg"); // RESULT - hardcoded password

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    Ok(client)
}

pub async fn async_mongo_opts() -> Result<mongodb::Client, mongodb::error::Error> {
    // FOR THIS CODE TO RUN
    // THE CARGO.toml "tokio-sync" feature should be turned off

    use mongodb::{options::ClientOptions, Client, };

    // Parse a connection string into an options struct.
    let mut client_options: mongodb::options::ClientOptions =
        mongodb::options::ClientOptions::parse("mongodb://localhost:27017").await.expect("msg");


    client_options.credential = Some(
        mongodb::options::Credential::builder()
            .username("user".to_owned())
            .password("HARDCODED_PASSWORD".to_owned()) // EMPTY PASSWORD
            .build()
    );
    
    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    Ok(client)
}

pub fn diesel_pg_uri() -> diesel::pg::PgConnection {
    use diesel::Connection;
    
    // #1
    let database_url = format!(
        "postgres://postgres:{}@localhost/diesel_db",
        "HARDCODED_PASSWORD"
    ); // "HARDCODED PASSWORD" is embedded in the connection string
    diesel::PgConnection::establish(&database_url) // RESULT #1
        .expect("msg");

    // #2
    diesel::PgConnection::establish("postgres://postgres:HARDCODED_PASSWORD@localhost/diesel_db") // RESULT #2
        .expect("msg")
}

pub fn diesel_mysql_uri() -> diesel::MysqlConnection {
    use diesel::mysql::MysqlConnection;
    use diesel::Connection;
    // #1
    let database_url = format!("mysql://root:{}@localhost/aaa", "HARDCODED_PASSWORD"); // "HARDCODED PASSWORD" is embedded in the connection string
    diesel::MysqlConnection::establish(&database_url) // RESULT #1
        .expect("msg");

    // #2
    diesel::MysqlConnection::establish("mysql://root:HARDCODED_PASSWORD@localhost/aaa") // RESULT #2
        .expect("msg")
}

pub async fn sqlx_pg_uri() -> Result<String, sqlx::Error> {
    use sqlx::postgres::PgConnectOptions;
    use sqlx::postgres::PgPool;
    use sqlx::Connection;

    // following line takes an hardcoded string and embeds it inside the connection string:
    let database_url_hardcoded = format!("postgres://postgres:{}@localhost/diesel_db", "HARDCODED_PASSWORD");
    //let database_url_empty = format!("postgres://postgres:{}@localhost/diesel_db", "");

    // #1:
    let mut con_hardcoded = sqlx::PgPool::connect(database_url_hardcoded.as_str()).await?; // RESULT #1
    //let mut con_empty = sqlx::PgPool::connect(database_url_hardcoded.as_str()).await?; // RESULT #2

    // #2:
    // URL connection string
    let con1 = sqlx::PgConnection::connect(&database_url_hardcoded).await?; // RESULT #2

    // #3:
    // URL connection string
    let con1 =
        sqlx::PgConnection::connect("postgres://postgres:HARDCODED_PASSWORD@localhost/diesel_db").await?; // RESULT #3

    // #4:
    // Parsing an object from the same string
    let mut opts: PgConnectOptions = database_url_hardcoded.parse()?;
    let pool = PgPool::connect_with(opts).await?; // RESULT #4

    Ok("conn".to_string())
}

pub async fn sqlx_pg_opts() -> Result<String, sqlx::Error> {
    use sqlx::postgres::PgConnectOptions;
    use sqlx::postgres::PgPool;
    use sqlx::postgres::PgSslMode;
    // Manually-constructed options
    let conn = sqlx::ConnectOptions::connect(
        &PgConnectOptions::new()
            .host("localhost")
            .port(2525)
            .username("postgres")
            .password("HARDCODED_PASSWORD") // RESULT #1
            .ssl_mode(PgSslMode::Require),
    )
    .await?;

    Ok("conn".to_string())
}

pub async fn sqlx_mysql_uri() -> Result<String, sqlx::Error> {
    use sqlx::mysql::{MySqlConnectOptions, MySqlConnection, MySqlPool};
    use sqlx::Connection;

    let conn = MySqlConnection::connect("mysql://root:HARDCODED_PASSWORD@localhost/aaa").await?; // RESULT #1

    let mut opts: MySqlConnectOptions = "mysql://root:HARDCODED_PASSWORD@localhost/aaa".parse()?; // RESULT #2
    let pool = MySqlPool::connect_with(opts).await?;

    Ok("conn".to_string())
}

pub async fn sqlx_mysql_opts() -> Result<String, sqlx::Error> {
    use sqlx::mysql::MySqlConnectOptions;
    use sqlx::ConnectOptions;

    // Manually-constructed options
    let conn = MySqlConnectOptions::new()
        .host("localhost")
        .username("root")
        .password("HARDCODED_PASSWORD") // RESULT #1
        .database("aaa")
        .connect()
        .await?;

    Ok("conn".to_string())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), sqlx::Error> {
    // let s = diesel_mysql_uri();
    sync_mongo_uri();
    async_mongo_uri();
    async_mongo_opts();
    diesel_pg_uri();
    diesel_mysql_uri();
    sqlx_pg_uri();
    sqlx_pg_opts();
    sqlx_mysql_uri();
    sqlx_mysql_opts();
    Ok(())
}
