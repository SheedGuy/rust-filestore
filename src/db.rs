use sqlx::postgres::{PgPool, PgPoolOptions};
use anyhow::Result;

pub async fn connect(url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
    .connect(url)
    .await?;

    Ok(pool)
}