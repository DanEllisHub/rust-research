use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::Row;

use std::env;
use std::ops::DerefMut;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let default_dsn = "mysql://root:password@localhost/bookstore";

    let pool = MySqlPoolOptions::new()
        .max_connections(200) 
        .min_connections(5) 
        .connect(&default_dsn)
        .await?;

    let sql = r#"insert into book  (title,author,price) value(?,?,?)"#;
    // Sink -- mising rollback and handling of errors and sucess
    let mut tx = pool.begin().await?; 
    let affect_rows = sqlx::query(sql)
        .bind("lisi")
        .bind("lisi")
        .bind(2222)

        .execute(tx.deref_mut())
        .await?;

        tx.commit().await?;
        let id = affect_rows.last_insert_id(); 
        println!("id = {}", id);
   

    Ok(())
}

#[tokio::main]
async fn read(account_id: i64) -> Result<(), sqlx::Error> {
    let pool = PgPool::connect("postgres://postgres:password@localhost:5432/postgres").await?;
    // Sink - Begin of the transaction and missing rollback
    let mut tx = pool.begin().await?;
    let _ = sqlx::query!("UPDATE users SET name = 'Mark' WHERE id = $1", account_id)
    .execute(&mut *tx)
    .await?;
        
        tx.commit().await?;
    Ok(())
    }


async fn read2(account_id: i64) -> Result<(), sqlx::Error> {
    let pool = PgPool::connect("postgres://postgres:password@localhost:5432/postgres").await?;
    // Sink - Missing commit and rollback
    let mut tx = pool.begin().await?;
    let _ = sqlx::query!("UPDATE users SET name = 'Thomasack' WHERE id = $1", account_id)
    .execute(&mut *tx)
    .await?;
        
    Ok(())
}

