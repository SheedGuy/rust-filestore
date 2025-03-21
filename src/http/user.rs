use axum::routing::{get, post};
use axum::Json;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Router,
};
use uuid::Uuid;

use crate::context::TheGoods;
use crate::data::users::*;
use crate::domain::{
    organizations::Organization,
    users::{CreateUser, User},
};

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
    Path((org_slug, id)): Path<(String, Uuid)>,
) -> ApiResult<Json<ApiResponse<User>>> {
    let user = get_user_data(&ctx.db, id).await?;

    // psuedo auth
    if user.organization == Organization::from_slug(&ctx, &org_slug).await? {
        Ok(Json(ApiResponse { data: user }))
    } else {
        Err(ApiError::Unauthorized)
    }
}

pub async fn update_user(
    State(ctx): State<TheGoods>,
    Path((org_slug, id)): Path<(String, Uuid)>,
    Json(payload): Json<CreateUser>,
) -> ApiResult<StatusCode> {
    let mut tx = ctx.db.begin().await?;

    let result = update_user_data(&mut tx, id, payload).await?;

    // holy unoptimized
    let user = get_user_data(&ctx.db, id).await?;
    // psuedo auth
    if user.organization == Organization::from_slug(&ctx, &org_slug).await? {
        // in theory I should only ever get 1
        match result {
            0 => {
                tx.rollback().await?;
                Err(ApiError::BadRequest(
                    format!("Zero rows affected when trying to update user with id: {id}")
                        .to_string(),
                ))
            }
            1 => {
                tx.commit().await?;
                Ok(StatusCode::NO_CONTENT)
            }
            _ => {
                tx.rollback().await?;
                Err(ApiError::BadRequest(
                    format!("{result} rows affected when trying to update user with id: {id}")
                        .to_string(),
                ))
            }
        }
    } else {
        tx.rollback().await?;
        Err(ApiError::Unauthorized)
    }
}

pub async fn create_user(
    State(ctx): State<TheGoods>,
    Path(org_slug): Path<String>,
    Json(payload): Json<CreateUser>,
) -> ApiResult<StatusCode> {
    let user_org = Organization::from_slug(&ctx, &org_slug).await?;

    let new_user_obj = payload.to_user(user_org);

    create_new_user(&ctx.db, new_user_obj).await?;

    Ok(StatusCode::CREATED)
}

pub async fn get_org_users(
    State(ctx): State<TheGoods>,
    Path(org_slug): Path<String>,
) -> ApiResult<Json<ApiResponse<Vec<User>>>> {
    let targ_org = Organization::from_slug(&ctx, &org_slug).await?;

    Ok(Json(ApiResponse {
        data: list_org_users(&ctx.db, targ_org).await?,
    }))
}
