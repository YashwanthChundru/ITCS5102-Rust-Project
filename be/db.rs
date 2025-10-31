use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub type Db = Pool<Sqlite>;

pub async fn init_pool() -> Result<Db> {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://database/job_portal.db".to_string());
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;
    Ok(pool)
}
