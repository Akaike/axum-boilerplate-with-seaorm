use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::DbErr;
use serde_json::json;
use thiserror::Error as ThisError;
use validator::ValidationErrors;

#[derive(ThisError, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Api(#[from] ApiError),

    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Database(#[from] DatabaseError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Api(e) => e.into_response(),
            Error::Auth(e) => e.into_response(),
            Error::Database(e) => e.into_response(),
        }
    }
}


#[derive(ThisError, Debug)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation failed: {0}")]
    ValidationError(#[from] ValidationErrors),

    #[error("Not found")]
    NotFound,

    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::ValidationError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::RequestFailed(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::NotFound => (StatusCode::NOT_FOUND, "Resource not found".into()),
            Self::ParseError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
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

#[derive(ThisError, Debug)]
pub enum AuthError {
    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("JWKS error: {0}")]
    JwksError(String),

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

#[derive(ThisError, Debug)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    Query(String),

    #[error("Record not found")]
    NotFound,
}

impl From<DbErr> for DatabaseError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(_) => Self::NotFound,
            _ => Self::Query(err.to_string()),
        }
    }
}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error occurred".into(),
            ),
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
