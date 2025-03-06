use uuid::Uuid;

use super::organization::Organization;

struct User {
    fname: String,
    lname: String,
    email: String,
    avatar_id: Option<Uuid>,
    organization: Organization,
}