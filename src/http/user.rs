use axum::{extract::{Path, State}, http::StatusCode, Router};
use axum::routing::{post, get};
use axum::Json;
use uuid::Uuid;

use crate::domain::{organizations::Organization, users::{CreateUser, User}};
use crate::context::TheGoods;
use crate::data::users::*;

use super::{ApiError, ApiResponse, ApiResult};


pub fn router() -> Router<TheGoods> {
    Router::new()
    .route("/{org_slug}/user/{id}", get(get_user).post(update_user))
    .route("/{org_slug}/user/new", post(create_user))
    .route("/{org_slug}/users", get(get_org_users))
}

#[axum_macros::debug_handler]
pub async fn get_user(
    State(ctx): State<TheGoods>,
    Path((org_slug, id)): Path<(String, Uuid)>
) -> ApiResult<Json<ApiResponse<User>>> {
    let user = get_user_data(&ctx.db, id).await?;

    let sender_org = Organization::from_slug(ctx, &org_slug).await?;

    // psuedo auth
    if user.organization == sender_org {
        Ok(Json(
            ApiResponse {
                data: user
            }
        ))
    } else {
        Err(ApiError::Unauthorized)
    }
}

pub async fn update_user(
    State(ctx): State<TheGoods>,
    Path(org_slug): Path<String>,
    Json(payload): Json<CreateUser>
) -> ApiResult<StatusCode> {

}

pub async fn create_user(
    State(ctx): State<TheGoods>,
    Path(org_slug): Path<String>,
    Json(payload): Json<CreateUser>
) -> ApiResult<StatusCode> {

}

pub async fn get_org_users(
    State(ctx): State<TheGoods>,
    Path(org_slug): Path<String>
) -> ApiResult<Json<ApiResponse<Vec<User>>>> {
    
}

// TODO:
//   Write remaining functions