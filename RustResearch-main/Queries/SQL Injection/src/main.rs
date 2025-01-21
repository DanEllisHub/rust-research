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
use diesel::{pg::PgConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use rand::distributions::{Distribution, Uniform};
use schema::posts;
use diesel::prelude::*;
use sqlx::Execute;
use sqlx::Postgres;
use sqlx::Row;
use sqlx::query;
use self::models::{NewPost, Post};
use sqlx::postgres::PgPool;
use std::env;

pub fn connect_diesel() -> PgConnection {
    //dotenv().ok();

    let database_url = "postgres://postgres:postgres@localhost/diesel_db";
    diesel::PgConnection::establish(&database_url).expect("msg")
}

pub async fn connect_sqlx() -> Result<sqlx::Pool<Postgres>, sqlx::Error> {
    let database_url = "postgres://postgres:postgres@localhost/diesel_db";
    let s= sqlx::PgPool::connect(database_url).await?;
    Ok(s)
}

pub fn sql_injection_vuln_1_diesel(input: String, conn: &mut PgConnection) -> String {
    let some = sql_query(input) // RESULT #1
    .sql("AND 1=1") // SINK #2
    .load::<Post>(conn);
    // ...

    "".to_string()
}

pub fn sql_injection_vuln_2_diesel(input: String, conn: &mut PgConnection) -> String {
    let some = sql_query("SELECT * FROM posts WHERE id = 1")
    .sql(input) // RESULT #2
    .load::<Post>(conn);
    // ...

    "".to_string()
}

pub fn sql_injection_vuln_3_diesel(input: String, conn: &mut PgConnection) {
    use self::schema::posts::dsl::*;

    let query = posts
    .select(title)
    .filter(
        sql::<diesel::sql_types::Bool>(&input) // RESULT
        .sql(" AND 1 > 2")
    )
    .get_results(conn);
    let expected: Vec<String> = query.unwrap();

    for element in expected.iter() {
        println!("{}", element);
    }
}

pub fn sql_injection_vuln_4_diesel(input: String, conn: &mut PgConnection) {
    use self::schema::posts::dsl::*;

    let query = posts
    .select(title)
    .filter(
        sql::<diesel::sql_types::Bool>("2 > 1")
        .sql(input.as_str()) // RESULT
    )
    .get_results(conn);
    let expected: Vec<String> = query.unwrap();

    for element in expected.iter() {
        println!("{}", element);
    }
}

pub fn sql_safe_diesel(input: String, conn: &mut PgConnection) {
    let check = sql_query("INSERT INTO posts (title, body, published) VALUES ($1, $2, $3)")
    .bind::<diesel::sql_types::Text, _>(&input) // NOT A RESULT - FALSE POSITIVE
    .bind::<diesel::sql_types::Text, _>(&input) // NOT A RESULT - FALSE POSITIVE
    .load::<Post>(conn);
}


pub async fn sql_injection_vuln_1_sqlx(input: String, pool: &PgPool) -> Result<String, sqlx::Error>{
    let mut data = sqlx::query_as::<_, Post>(&input) // RESULT #1
    .bind(&input) // NOT A RESULT - Potential False Positive
    .fetch_all(pool)
    .await?;

    for rec in data {
        println!("{}", rec.title);
    }

    Ok("done".to_string())
}

pub async fn sql_injection_vuln_2_sqlx(input: String, pool: &PgPool) -> Result<String, sqlx::Error>{
    let mut data = sqlx::query(&input) // RESULT #1
    .bind(&input) // NOT A RESULT - Potential False Positive
    .fetch_all(pool)
    .await?;

    Ok("done".to_string())
}

pub async fn sql_injection_vuln_3_sqlx(input: String, pool: &PgPool) -> Result<String, sqlx::Error>{
    let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
        &input // RESULT #1
    );
    
    query_builder.separated(" ").push_unseparated(&input); // RESULT #2
    
    query_builder.push(&input); // RESULT #3
    let mut query = query_builder.build();

    let recs = query.execute(pool)
    .await?;

    Ok("done".to_string())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), sqlx::Error>{
    
    //sql_injection_vuln_4_diesel(" AND 1 < 0".to_string(), &mut connect_diesel());

    let database_url = "postgres://postgres:postgres@localhost/diesel_db";
    let s= sqlx::PgPool::connect(database_url).await?;
    sql_injection_vuln_3_sqlx("".to_string(), &s).await?;
    
    Ok(())
}