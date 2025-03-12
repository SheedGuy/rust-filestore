use anyhow::Ok;
use axum::http::StatusCode;
use sqlx::{any, PgPool};
use uuid::Uuid;

use crate::domain::organizations::Organization;
use crate::domain::users::{CreateUser, User};
use crate::http::ApiResult;

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
        WHERE u.user_id = $1"#,
    )
    .bind(user_id)
    .fetch_one(conn)
    .await?)
}

pub async fn update_user_data(
    conn: &PgPool,
    user_id: Uuid,
    updates: CreateUser,
) -> anyhow::Result<u64> {
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
    )
    .execute(conn)
    .await?
    .rows_affected())
}

pub async fn create_new_user(conn: &PgPool, new_user: User) -> anyhow::Result<()> {
    // dont actually need transactions here, just playing with them
    let mut tx = conn.begin().await?;

    sqlx::query!(
        r#"
        insert into "users"
        (user_id, f_name, l_name, email, org_id)
        values ($1, $2, $3, $4, $5)
        "#,
        new_user.user_id,
        new_user.f_name,
        new_user.l_name,
        new_user.email,
        new_user.organization.org_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn list_org_users(conn: &PgPool, org: Organization) -> anyhow::Result<Vec<User>> {
    Ok(sqlx::query_as::<_, User>(
        r#"
            SELECT 
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
            WHERE u.org_id = $1 
            "#,
    )
    .bind(org.org_id)
    .fetch_all(conn)
    .await?)
}
