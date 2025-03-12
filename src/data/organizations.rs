use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::organizations::Organization;

pub async fn get_org_data_by_slug(conn: &PgPool, slug: &str) -> anyhow::Result<Organization> {
    Ok(
        sqlx::query_as!(
            Organization,
            r#"
            SELECT
                org_id,
                org_name as name,
                slug,
                bucket_name
            from "organizations"
            where slug = $1
            "#,
            slug.clone()
        )
        .fetch_one(conn)
        .await?
    )
}

pub async fn get_org_data_by_id(conn: &PgPool, org_id: Uuid) -> anyhow::Result<Organization> {
    Ok(sqlx::query_as!(
        Organization,
        r#"
        Select org_id, org_name as  name, slug, bucket_name from "organizations"
        where org_id = $1
        "#,
        org_id
        ).fetch_one(conn).await?
    )
}

pub async fn create_and_return(conn: &PgPool, new_org: Organization) -> anyhow::Result<Organization> {
    sqlx::query!(
        r#"
        insert into "organizations"
        (org_id, org_name, slug, bucket_name)
        values ($1, $2, $3, $4)
        "#,
        new_org.org_id,
        new_org.name,
        new_org.slug,
        new_org.bucket_name
    ).execute(conn).await?;


    get_org_data_by_id(conn, new_org.org_id).await
}