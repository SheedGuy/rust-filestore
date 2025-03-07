// use anyhow::{Error, Result};
use uuid::Uuid;
use serde::Deserialize;

pub struct Organization {
    org_id: Uuid,
    name: String,
    slug: String,
    bucket_name: String
}

#[derive(Deserialize)]
pub struct CreateOrganization {
    name: String,
    slug: String
}