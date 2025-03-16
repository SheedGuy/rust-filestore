use uuid::Uuid;

use crate::http::ApiError;

#[derive(sqlx::Type, serde::Serialize, serde::Deserialize, Clone)]
#[sqlx(type_name = "media_purpose", rename_all = "lowercase")]
pub enum MediaPurpose {
    Avatar,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Media {
    pub media_id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub media_purpose: MediaPurpose,
}

pub fn is_image_type(content_type: &str) -> Result<String, ApiError> {
    let derived_type: mime::Mime = content_type.parse().unwrap();

    match (derived_type.type_(), derived_type.subtype()) {
        (mime::IMAGE, mime::PNG) => Ok(derived_type.to_string()),
        (mime::IMAGE, mime::JPEG) => Ok(derived_type.to_string()),
        (mime::IMAGE, _) => Err(ApiError::BadRequest(
            "Unsupported image type. Please use a .jpg or .png".to_string(),
        )),
        (_, _) => Err(ApiError::BadRequest(
            "Not an image. Please upload a .jpg or .png file.".to_string(),
        )),
    }
}
