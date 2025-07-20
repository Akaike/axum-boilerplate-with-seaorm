use std::borrow::Cow;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

use super::{database::DatabaseError, service::ServiceError};

pub type ApiResult<T> = Result<T, ApiError>;

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

impl From<ValidationErrors> for ApiError {
    fn from(err: ValidationErrors) -> Self {
        ApiError::BadRequest(err.to_string())
    }
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::NotFound => ApiError::NotFound("resource not found"),
            ServiceError::AlreadyExists => ApiError::Conflict("resource already exists"),
            ServiceError::ValidationError => ApiError::BadRequest("validation failed".into()),
            ServiceError::DatabaseError(db) => match db {
                DatabaseError::NotFound => ApiError::NotFound("resource not found"),
                DatabaseError::Conflict => ApiError::Conflict("resource already exists"),
                DatabaseError::OperationFailed => ApiError::BadRequest("operation failed".into()),
                DatabaseError::Internal => {
                    ApiError::Internal(anyhow::anyhow!("database internal error"))
                }
            },
        }
    }
}

impl ApiError {
    pub fn status(&self) -> StatusCode {
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

    #[test]
    fn validation_maps_to_bad_request() {
        let mut ve = ValidationErrors::new();
        ve.add("field", validator::ValidationError::new("invalid"));
        let api_err: ApiError = ve.into();

        assert_eq!(api_err.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn service_error_maps_to_correct_status() {
        let service_err = ServiceError::NotFound;
        let api_err: ApiError = service_err.into();
        assert_eq!(api_err.status(), StatusCode::NOT_FOUND);
    }
}
