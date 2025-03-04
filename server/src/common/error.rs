use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::DbErr;
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

pub type ApiResult<T> = Result<T, ApiError>;
pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
    
    #[error("Service error: {0}")]
    ServiceError(#[from] ServiceError),

    #[error("Validation failed: {0}")]
    ValidationError(#[from] ValidationErrors),
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    Query(String),

    #[error("Record not found")]
    NotFound,

    #[error("Error during insert")]
    Insert,

    #[error("A record with this identifier already exists")]
    UniqueViolation,

    #[error("Internal database error")]
    Internal(String),
}

impl From<DbErr> for DatabaseError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(_) => DatabaseError::NotFound,
            DbErr::Query(msg) => {
                if msg.to_string().contains("unique") || msg.to_string().contains("duplicate") {
                    return DatabaseError::UniqueViolation
                }

                eprintln!("Database error: {}", msg);
                DatabaseError::Query("Database operation failed".to_string())
            }
            DbErr::Exec(_) => DatabaseError::Query("Database execution error".to_string()),
            DbErr::Conn(_) => DatabaseError::Query("Database connection error".to_string()),
            DbErr::RecordNotInserted => DatabaseError::Insert,
            _ => {
                eprintln!("Unexpected database error: {:?}", err);
                DatabaseError::Internal(err.to_string())
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Self::NotFound(message) => (StatusCode::NOT_FOUND, message),
            Self::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
            Self::Forbidden(message) => (StatusCode::FORBIDDEN, message),
            Self::Conflict(message) => (StatusCode::CONFLICT, message),
            Self::ValidationError(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            Self::Internal(err) => {
                eprintln!("Internal server error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                )
            }
            Self::ServiceError(err) => match err {
                ServiceError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
                ServiceError::AlreadyExists(msg) => (StatusCode::CONFLICT, msg),
                ServiceError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
                ServiceError::DatabaseError(db_err) => match db_err {
                    DatabaseError::Query(msg) => (StatusCode::BAD_REQUEST, msg),
                    DatabaseError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
                    DatabaseError::Insert => (StatusCode::BAD_REQUEST, "Operation failed".to_string()),
                    DatabaseError::UniqueViolation => (
                        StatusCode::CONFLICT, 
                        "Request could not be completed. Please try again with different credentials".to_string()
                    ),
                    DatabaseError::Internal(_) => (
                        StatusCode::INTERNAL_SERVER_ERROR, 
                        "An internal error occurred".to_string()
                    ),
                }
            }
        };

        (status, Json(json!({
            "error": {
                "message": error_message,
                "status": status.as_u16()
            }
        }))).into_response()
    }
}
