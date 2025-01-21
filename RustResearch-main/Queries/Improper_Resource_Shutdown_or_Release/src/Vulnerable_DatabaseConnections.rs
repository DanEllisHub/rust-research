use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::Row;
use sqlx::PgPool;
use std::ops::DerefMut;
use std::time::Duration;
use tokio;


#[tokio::main]
async fn update(account_id: i64) -> Result<(), sqlx::Error> {

    let pool = PgPool::connect("postgres://postgres:password@localhost:5432/postgres").await?;
    let mut tx = pool.begin().await?;
    let _ = sqlx::query!("UPDATE users SET name = 'Mark2' WHERE id = $1", account_id)
    .execute(&mut *tx)
    .await?;
            
    tx.commit().await?;

    other_actions_not_related_to_the_database_conection();
    Ok(())
}