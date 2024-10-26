use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::DbErr;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ApiError {
    #[error("Not found error")]
    NotFound,

    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!("Error occurred: {:?}", self);

        let (status, error_message) = match self {
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                "The requested resource was not found",
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred",
            ),
        };

        let body = Json(json!({
            "status": "error",
            "error": {
                "code": status.as_u16(),
                "message": error_message,
            }
        }));

        (status, body).into_response()
    }
}

impl From<DbErr> for ApiError {
    fn from(err: DbErr) -> Self {
        tracing::error!("Database error occurred: {:?}", err);

        ApiError::InternalServerError
    }
}
