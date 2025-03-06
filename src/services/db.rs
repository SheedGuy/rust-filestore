use sqlx::postgres::{PgPool, PgPoolOptions};
// use sqlx::{Acquire, Postgres};
use sqlx::PgExecutor;
use anyhow::Result;

// pub trait PgAcquire<'c>: Acquire<'c, Database = Postgres> {}

// impl<'c, T> PgAcquire<'c> for T where T: Acquire<'c, Database = Postgres> {}

pub async fn connect(url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
    .connect(url)
    .await?;

    Ok(pool)
}

// pub async fn get_users<'c>(conn: &impl PgExecutor<'c>) {
// }

// TODO:
// - Set up DB schem
// - Set up migration folder with db schema creation
// - Make database functions
    // - CRUD user