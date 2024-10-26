use axum::{
    extract::rejection::JsonRejection,
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
    #[error("Not found error")]
    NotFound,

    #[error("Internal server error: {0}")]
    InternalServerError(#[from] anyhow::Error),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("Error occurred: {:?}", self);

        let (status, error_message) = match self {
            Error::NotFound => (
                StatusCode::NOT_FOUND,
                "The requested resource was not found".into(),
            ),
            Error::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred".into(),
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

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(_) => Error::NotFound,
            _ => Error::InternalServerError(err.into()),
        }
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        let message = errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                let errors = errors
                    .iter()
                    .map(|err| {
                        err.message
                            .clone()
                            .map_or_else(|| "Invalid value".to_string(), |msg| msg.to_string())
                    })
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{}: {}", field, errors)
            })
            .collect::<Vec<String>>()
            .join(", ");

        Error::BadRequest(message)
    }
}

impl From<JsonRejection> for Error {
    fn from(rejection: JsonRejection) -> Self {
        match rejection {
            JsonRejection::MissingJsonContentType(_) => Error::BadRequest(
                "Expected request body to be in JSON format (Content-Type: application/json)"
                    .into(),
            ),
            JsonRejection::JsonSyntaxError(_) => {
                Error::BadRequest("The JSON payload is malformed or contains syntax errors".into())
            }
            JsonRejection::JsonDataError(_) => {
                Error::BadRequest("The JSON payload contains invalid data or types".into())
            }
            JsonRejection::BytesRejection(_) => {
                Error::BadRequest("Failed to read the request body".into())
            }
            _ => Error::InternalServerError(anyhow::anyhow!(
                "An unknown error occurred while processing the JSON"
            )),
        }
    }
}
