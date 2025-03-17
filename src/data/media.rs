use sqlx::{PgPool, Postgres, Result, Transaction};
use uuid::Uuid;

use crate::domain::media::{Media, MediaPurpose};

pub async fn insert_media_obj(tx: &mut Transaction<'_, Postgres>, media: &Media) -> Result<()> {
    sqlx::query!(
        r#"
        insert into "media" (media_id, file_name, content_type, media_purpose)
        values ($1, $2, $3, $4)
        "#,
        media.media_id,
        media.file_name,
        media.content_type,
        media.media_purpose.clone() as MediaPurpose // using types sucks actually
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn get_media_obj(conn: &PgPool, media_id: Uuid) -> Result<Media> {
    Ok(sqlx::query_as!(
        Media,
        r#"
        Select media_id, file_name, content_type, media_purpose as "media_purpose: MediaPurpose"  from "media"
        where media_id = $1
        "#,
        media_id
    ).fetch_one(conn)
    .await?)
}
