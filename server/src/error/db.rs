use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::DbErr;
use serde_json::json;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    Query(String),

    #[error("Record not found")]
    NotFound,
}

impl From<DbErr> for DbError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(_) => Self::NotFound,
            _ => Self::Query(err.to_string()),
        }
    }
}

impl IntoResponse for DbError {
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
