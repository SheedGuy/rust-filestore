use sqlx::PgPool;
use uuid::Uuid;

use crate::http::ApiResult;
use crate::domain::users::*;


// struct UserRow {
//     user_id: Option<Uuid>,
//     f_name: Option<String>,
//     l_name: Option<String>,
//     email: Option<String>,
//     avatar_id: Option<Uuid>,

//     org_id: Option<Uuid>,
//     org_name: Option<String>,
//     slug: Option<String>,
//     bucket_name: Option<String>,   
// }

pub async fn get_user_data(conn: &PgPool, user_id: Uuid) -> anyhow::Result<User> {
    Ok(sqlx::query_as::<_, User>(
        r#"SELECT 
            u.user_id, 
            u.f_name, 
            u.l_name, 
            u.email, 
            u.avatar_id, 
            o.org_id,
            o.org_name,
            o.slug,
            o.bucket_name
        from
            "users" u
        INNER JOIN
            "organizations" o
        USING  (org_id)
        WHERE u.user_id = $1"#)
        .bind(user_id)
        .fetch_one(conn)
        .await?)
}