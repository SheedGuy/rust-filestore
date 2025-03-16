use sqlx::{Postgres, Result, Transaction};

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
