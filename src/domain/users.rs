use uuid::Uuid;
use serde::{Deserialize, Serialize};

use super::organizations::Organization;

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    user_id: Uuid,
    f_name: String,
    l_name: String,
    email: String,
    avatar_id: Option<Uuid>,
    #[sqlx(flatten)]
    pub organization: Organization,
}

#[derive(Deserialize)]
pub struct CreateUser {
    f_name: String,
    l_name: String,
    email: String
}