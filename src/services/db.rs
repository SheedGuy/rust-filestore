use sqlx::postgres::{PgPool, PgPoolOptions};
// use sqlx::{Acquire, Postgres};
use anyhow::Result;

// pub trait PgAcquire<'c>: Acquire<'c, Database = Postgres> {}

// impl<'c, T> PgAcquire<'c> for T where T: Acquire<'c, Database = Postgres> {}

pub async fn connect(url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
    .connect(url)
    .await?;

    Ok(pool)
}