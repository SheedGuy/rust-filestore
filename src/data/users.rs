use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::http::ApiResult;
use crate::domain::users::{User, CreateUser};


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


pub async fn update_user_data(conn: &PgPool, user_id: Uuid, updates: CreateUser) -> anyhow::Result<u64> {

    // this should be rewritten better
    Ok(sqlx::query!(
        r#"
        update "users"
        set
            f_name = $1,
            l_name = $2,
            email = $3
        where user_id = $4
        "#,
        updates.f_name,
        updates.l_name,
        updates.email,
        user_id
    ).execute(conn).await?.rows_affected())

}