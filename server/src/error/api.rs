use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error as ThisError;
use validator::ValidationErrors;

#[derive(ThisError, Debug)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation failed: {0}")]
    ValidationError(#[from] ValidationErrors),

    #[error("Not found")]
    NotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::ValidationError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::NotFound => (StatusCode::NOT_FOUND, "Resource not found".into()),
        };

        let body = Json(json!({
            "status": "error",
            "error": {
                "code": status.as_u16(),
                "message": message,
            }
        }));

        (status, body).into_response()
    }
}
