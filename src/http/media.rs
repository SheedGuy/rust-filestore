use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::post;
use axum::Router;
use uuid::Uuid;

use crate::context::TheGoods;
use crate::data::media::{get_media_obj, insert_media_obj};
use crate::data::users::{get_user_data, update_user_avatar};
use crate::domain::media::{self, Media, MediaPurpose};

use super::{ApiError, ApiResult};

pub fn router() -> Router<TheGoods> {
    Router::new().route("/{user_id}/avatar", post(upload_avatar).get(get_avatar))
}

pub async fn upload_avatar(
    State(ctx): State<TheGoods>,
    Path(user_id): Path<Uuid>,
    mut file_upload: Multipart,
) -> ApiResult<StatusCode> {
    // Can store anything we want in form-data. Could move userID there. Too lazy for now

    println!("made it to server");

    let user = get_user_data(&ctx.db, user_id).await?;

    if let Some(field) = file_upload.next_field().await? {
        let new_avatar = Media {
            content_type: media::is_image_type(field.content_type().unwrap())?,
            file_name: field.file_name().unwrap().to_string(),
            media_id: Uuid::new_v4(),
            media_purpose: MediaPurpose::Avatar,
        };

        let mut tx = ctx.db.begin().await?;

        insert_media_obj(&mut tx, &new_avatar).await?;

        update_user_avatar(&mut tx, &user, new_avatar.media_id).await?;

        ctx.gcs
            .upload_image(
                &new_avatar.file_name,
                field.bytes().await?.to_vec(),
                &new_avatar.content_type,
                &user.organization.bucket_name,
            )
            .await?;

        tx.commit().await?;

        Ok(StatusCode::CREATED)
    } else {
        Err(ApiError::BadRequest(
            "No multipart form data or image sent".to_string(),
        ))
    }
}

pub async fn get_avatar(
    State(ctx): State<TheGoods>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<Response> {
    let user = get_user_data(&ctx.db, user_id).await?;

    match user.avatar_id {
        Some(id) => {
            let avatar = get_media_obj(&ctx.db, id).await?;
            Ok(ctx
                .gcs
                .get_image_stream(&avatar, &user.organization.bucket_name)
                .await?)
        }
        None => Err(ApiError::BadRequest(
            "User does not have an avatar".to_string(),
        )),
    }
}
