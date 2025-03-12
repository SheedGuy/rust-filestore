use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::organizations::Organization;

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub user_id: Uuid,
    pub f_name: String,
    pub l_name: String,
    pub email: String,
    pub avatar_id: Option<Uuid>,
    #[sqlx(flatten)]
    pub organization: Organization,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub f_name: String,
    pub l_name: String,
    pub email: String,
}

impl CreateUser {
    pub fn to_user(self: Self, org: Organization) -> User {
        User {
            user_id: Uuid::new_v4(),
            f_name: self.f_name,
            l_name: self.l_name,
            email: self.email,
            avatar_id: None,
            organization: org,
        }
    }
}
