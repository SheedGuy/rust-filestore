use uuid::Uuid;
use serde::Deserialize;

use super::organizations::Organization;

pub struct User {
    user_id: Uuid,
    f_name: String,
    l_name: String,
    email: String,
    avatar_id: Option<Uuid>,
    organization: Organization,
}

#[derive(Deserialize)]
pub struct CreateUser {
    f_name: String,
    l_name: String,
    email: String
}