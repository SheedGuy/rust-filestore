use axum::extract::Path;
use axum::routing::{get, post};
use axum::Json;
use axum::{extract::State, http::StatusCode, Router};
use uuid::Uuid;

use crate::data::organizations::*;
use crate::domain::organizations::{Organization, UpdateOrg};
use crate::{context::TheGoods, domain::organizations::CreateOrganization};

use super::{ApiResponse, ApiResult};

pub fn router() -> Router<TheGoods> {
    Router::new()
        .route("/organization/create", post(create_org))
        .route("/organizations/list", get(get_all_orgs))
        .route("/{org_slug}", post(update_org).get(get_one_org))
}

pub async fn create_org(
    State(ctx): State<TheGoods>,
    Json(payload): Json<CreateOrganization>,
) -> ApiResult<Json<ApiResponse<Organization>>> {
    let mut bucket_name = payload.slug.clone();
    bucket_name.push_str("_media_assets");

    let new_org = Organization {
        org_id: Uuid::new_v4(),
        name: payload.name,
        slug: payload.slug,
        bucket_name,
    };

    Ok(Json(ApiResponse::new(
        create_and_return(&ctx.db, new_org).await?,
    )))
}

pub async fn get_all_orgs(
    State(ctx): State<TheGoods>,
) -> ApiResult<Json<ApiResponse<Vec<Organization>>>> {
    Ok(Json(ApiResponse::new(list_all_orgs(&ctx.db).await?)))
}

pub async fn get_one_org(
    State(ctx): State<TheGoods>,
    Path(org_slug): Path<String>,
) -> ApiResult<Json<ApiResponse<Organization>>> {
    Ok(Json(ApiResponse::new(
        get_org_data_by_slug(&ctx.db, &org_slug).await?,
    )))
}

pub async fn update_org(
    State(ctx): State<TheGoods>,
    Path(org_slug): Path<String>,
    Json(payload): Json<UpdateOrg>,
) -> ApiResult<StatusCode> {
    update_org_name(&ctx.db, &org_slug, &payload.name).await?;

    Ok(StatusCode::NO_CONTENT)
}
