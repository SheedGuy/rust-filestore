use axum::Router;
use axum::routing::{post, get};

use crate::context::TheGoods;

pub fn router() -> Router<TheGoods> {
    Router::new()
    .route("/organization/create", post(create_org))
    .route("/organizations/list", get(get_all_orgs))
    .route("/{org_slug}", post(update_org).get(get_org))
}