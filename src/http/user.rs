use axum::{extract::{Json, Path, State}, Router};
use axum::routing::{post, get};
use uuid::Uuid;

use crate::domain::users::CreateUser;
use crate::context::TheGoods;

pub fn router() -> Router<TheGoods> {
    Router::new()
    .route("/{org_slug}/user/{id}", get(get_user).post(update_user))
    .route("/{org_slug}/user/new", post(create_user))
    .route("/{org_slug}/users", get(get_org_users))
}

pub async fn get_user(
    State(ctx): State<TheGoods>,
    Path((org_slug, id)): Path<(String, Uuid)>
) {}

pub async fn update_user(
    State(ctx): State<TheGoods>,
    Path(org_slug): Path<String>,
    Json(payload): Json<CreateUser>
) {}

pub async fn create_user(
    State(ctx): State<TheGoods>,
    Path(org_slug): Path<String>,
    Json(payload): Json<CreateUser>
) {}

pub async fn get_org_users(
    State(ctx): State<TheGoods>,
    Path(org_slug): Path<String>
) {}

// TODO:
// Figure out response types for each
// Figure out database access