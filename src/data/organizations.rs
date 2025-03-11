use sqlx::PgPool;

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