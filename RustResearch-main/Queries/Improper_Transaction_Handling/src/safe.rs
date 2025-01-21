
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::Row;

use std::env;
use std::ops::DerefMut;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn read(account_id: i64) -> Result<(), sqlx::Error> {
    let pool = PgPool::connect("postgres://postgres:password@localhost:5432/postgres").await?;
    let mut tx = pool.begin().await?;

    let result = sqlx::query!("UPDATE users SET name = 'Mark' WHERE id = $1", account_id)
        .execute(&mut *tx)
        .await;
    // First Sanitizer - handling of errors and sucess
    match result {
        Ok(_) => {
            println!("Transaction executed successfully");
            // Second Sanitizer - Commit the transaction
            tx.commit().await?;
            println!("Transaction committed successfully");
        },
        Err(e) => {
            // Third Sanitizer - rollback implemented
            tx.rollback().await?;
            println!("Transaction rolled back due to error: {:?}", e);
        }
    }
    Ok(())
}



async fn main() -> Result<(), sqlx::Error> {
    let default_dsn = "mysql://root:password@localhost/bookstore";

    let pool = MySqlPoolOptions::new()
        .max_connections(200) 
        .min_connections(5) 
        .connect(&default_dsn)
        .await?;

    let sql = r#"insert into book (title,author,price) value(?,?,?)"#;
    let mut tx = pool.begin().await?; 
    let affect_rows = sqlx::query(sql)
        .bind("test")
        .bind("test")
        .bind(12)
        .execute(tx.deref_mut())
        .await?;
    // First Sanitizer - handling of errors and sucess
    if affect_rows.rows_affected() == 0 {
        // Second Sanitizer - rollback is presented
        tx.rollback().await?;
    } else {
        // Third Sanitizer - Commit implemented
        tx.commit().await?;
        let id = affect_rows.last_insert_id(); 
        println!("id = {}", id);
    }

    Ok(())
}