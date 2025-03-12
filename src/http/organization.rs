use axum::{extract::State, http::StatusCode, Router};
use axum::Json;
use axum::routing::{post, get};
use uuid::Uuid;

use crate::data::organizations::create_and_return;
use crate::domain::organizations::Organization;
use crate::{context::TheGoods, domain::organizations::CreateOrganization};

use super::{ApiResponse, ApiResult};

pub fn router() -> Router<TheGoods> {
    Router::new()
    .route("/organization/create", post(create_org))
//     .route("/organizations/list", get(get_all_orgs))
//     .route("/{org_slug}", post(update_org).get(get_org))
}

pub async fn create_org(
    State(ctx): State<TheGoods>,
    Json(payload): Json<CreateOrganization>
) -> ApiResult<Json<ApiResponse<Organization>>> {

    let mut bucket_name = payload.slug.clone();
    bucket_name.push_str("_media_assets");
    
    let new_org = Organization {
        org_id: Uuid::new_v4(),
        name: payload.name,
        slug: payload.slug,
        bucket_name
    };

    let inserted_org = create_and_return(&ctx.db, new_org).await?;

    Ok(Json(ApiResponse::new(inserted_org)))
}