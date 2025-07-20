use std::borrow::Cow;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::{DbErr, SqlErr};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

pub type ApiResult<T> = Result<T, ApiError>;
pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("unauthorized")]
    Unauthorized(&'static str),
    #[error("forbidden")]
    Forbidden(&'static str),
    #[error("not found")]
    NotFound(&'static str),
    #[error("bad request")]
    BadRequest(String),
    #[error("conflict")]
    Conflict(&'static str),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("resource not found")]
    NotFound,
    #[error("resource already exists")]
    AlreadyExists,
    #[error("validation error")]
    ValidationError,
    #[error("database error")]
    DatabaseError(#[from] DatabaseError),
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("record not found")]
    NotFound,
    #[error("unique constraint violation")]
    Conflict,
    #[error("operation failed")]
    OperationFailed,
    #[error("internal database error")]
    Internal,
}

impl From<DbErr> for DatabaseError {
    fn from(err: DbErr) -> Self {
        use DatabaseError::*;

        match &err {
            DbErr::RecordNotFound(_) => NotFound,
            _ if matches!(err.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) => Conflict,

            DbErr::ConnectionAcquire(_)
            | DbErr::Conn(_)
            | DbErr::RecordNotInserted
            | DbErr::RecordNotUpdated
            | DbErr::AttrNotSet(_)
            | DbErr::Query(_)
            | DbErr::Exec(_) => OperationFailed,

            _ => Internal,
        }
    }
}

impl From<DbErr> for ServiceError {
    fn from(err: DbErr) -> Self {
        ServiceError::DatabaseError(err.into())
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(err: ValidationErrors) -> Self {
        ApiError::BadRequest(err.to_string())
    }
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::NotFound => ApiError::NotFound("resource not found".into()),
            ServiceError::AlreadyExists => ApiError::Conflict("resource already exists".into()),
            ServiceError::ValidationError => ApiError::BadRequest("validation failed".into()),
            ServiceError::DatabaseError(db) => match db {
                DatabaseError::NotFound => ApiError::NotFound("resource not found".into()),
                DatabaseError::Conflict => ApiError::Conflict("resource already exists".into()),
                DatabaseError::OperationFailed => ApiError::BadRequest("operation failed".into()),
                DatabaseError::Internal => {
                    ApiError::Internal(anyhow::anyhow!("database internal error"))
                }
            },
        }
    }
}

impl ApiError {
    fn status(&self) -> StatusCode {
        use ApiError::*;
        match self {
            Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Forbidden(_) => StatusCode::FORBIDDEN,
            NotFound(_) => StatusCode::NOT_FOUND,
            BadRequest(_) => StatusCode::BAD_REQUEST,
            Conflict(_) => StatusCode::CONFLICT,
            Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn client_msg(&self) -> Cow<'static, str> {
        use ApiError::*;
        match self {
            Unauthorized(m) | Forbidden(m) | NotFound(m) | Conflict(m) => Cow::Borrowed(m),
            BadRequest(m) => Cow::Owned(m.clone()),
            Internal(_) => Cow::Borrowed("an internal error occurred"),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status();
        (
            status,
            Json(json!({
                "error": {
                    "status": status.as_u16(),
                    "message": self.client_msg(),
                }
            })),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::DbErr;

    #[test]
    fn db_err_maps_to_api_status() {
        let cases = vec![(
            DbErr::RecordNotFound("Record not found".to_string()),
            StatusCode::NOT_FOUND,
        )];

        for (db_err, expected_status) in cases {
            let service_err: ServiceError = db_err.into();
            let api_err: ApiError = service_err.into();
            assert_eq!(api_err.status(), expected_status);
        }
    }

    #[test]
    fn validation_maps_to_bad_request() {
        let mut ve = ValidationErrors::new();
        ve.add("field", validator::ValidationError::new("invalid"));
        let api_err: ApiError = ve.into();

        assert_eq!(api_err.status(), StatusCode::BAD_REQUEST);
    }
}
