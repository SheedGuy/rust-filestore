// use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::TheGoods;
use crate::data::organizations;

#[derive(sqlx::FromRow, PartialEq, Serialize)]
pub struct Organization {
    pub org_id: Uuid,
    pub name: String,
    pub slug: String,
    pub bucket_name: String,
}

#[derive(Deserialize)]
pub struct CreateOrganization {
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize)]
pub struct UpdateOrg {
    pub name: String,
}

impl Organization {
    pub async fn from_slug(ctx: &TheGoods, slug: &str) -> anyhow::Result<Self> {
        Ok(organizations::get_org_data_by_slug(&ctx.db, slug).await?)
    }
}
