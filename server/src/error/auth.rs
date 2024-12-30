use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum AuthError {
    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("JWKS error: {0}")]
    JwksError(#[from] jsonwebtoken::errors::Error),

    #[error("Missing token")]
    MissingToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = StatusCode::UNAUTHORIZED;
        let body = Json(json!({
            "status": "error",
            "error": {
                "code": status.as_u16(),
                "message": self.to_string(),
            }
        }));

        (status, body).into_response()
    }
}
