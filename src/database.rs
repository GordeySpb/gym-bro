use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type DbPool = Pool<Postgres>;

pub async fn create_poll(db_url: &str) -> Result<DbPool> {
    let pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await?;

    println!("Applying migrations...");
    sqlx::migrate!().run(&pool).await?;
    println!("Migrations applied successfully!");

    Ok(pool)
}
