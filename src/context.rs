use sqlx::PgPool;

#[derive(Clone)]
pub struct TheGoods {
    pub db: PgPool, // Add GCS when needed
}

impl TheGoods {
    pub async fn new(db: PgPool) -> Self {
        TheGoods { db }
    }
}
