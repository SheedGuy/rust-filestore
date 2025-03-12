pub mod organization;
pub mod user;

use axum::http::StatusCode;
use axum::response::Result;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use serde::Serialize;

use crate::context::TheGoods;

fn new(goodies: TheGoods) -> Router {
    Router::new()
        .merge(organization::router())
        .merge(user::router())
        .with_state(goodies)
}

pub async fn serve(goodie_bag: TheGoods, port: u16) -> anyhow::Result<()> {
    let app = new(goodie_bag);

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port)).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("unauthorized")]
    Unauthorized,

    #[error("not found")]
    NotFound,

    #[error("`{0}`")]
    BadRequest(String),

    #[error("sqlx: {0:?}")]
    Sqlx(#[from] sqlx::Error),

    #[error("internal: {0:?}")]
    Anyhow(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ApiErrorResponse {
    code: u16,
    detail: Option<String>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match &self {
            Self::Sqlx(ref e) => match e {
                sqlx::Error::RowNotFound => {
                    return StatusCode::NOT_FOUND.into_response();
                }

                _ => {
                    tracing::error!("SQLx error: {:?}", e)
                }
            },
            Self::Anyhow(_) => tracing::error!("{}", self),
            _ => tracing::warn!("{}", self),
        }

        Json(ApiErrorResponse {
            code: self.status_code().as_u16(),
            detail: self.details(),
        })
        .into_response()
    }
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn details(&self) -> Option<String> {
        match self {
            Self::BadRequest(detail) => Some(detail.clone()),
            _ => None,
        }
    }
}
