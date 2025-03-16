use sqlx::PgPool;

use crate::services::gcs::GCSClient;

#[derive(Clone)]
pub struct TheGoods {
    pub db: PgPool, // Add GCS when needed
    pub gcs: GCSClient,
}

impl TheGoods {
    pub fn new(db: PgPool, gcs: GCSClient) -> Self {
        TheGoods { db, gcs }
    }
}
