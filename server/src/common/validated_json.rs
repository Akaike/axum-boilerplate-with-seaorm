use std::ops::{Deref, DerefMut};

use axum::extract::{rejection::JsonRejection, FromRequest, Json, Request};
use serde::de::DeserializeOwned;
use tracing::debug;
use validator::Validate;

use crate::common::error::ApiError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T> Deref for ValidatedJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ValidatedJson<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await.map_err(|e| {
            debug!("JSON parsing error: {:?}", e);
            map_json_rejection_to_user_error(e)
        })?;

        value.validate()?;

        Ok(ValidatedJson(value))
    }
}

fn map_json_rejection_to_user_error(rejection: JsonRejection) -> ApiError {
    use axum::extract::rejection::*;

    match rejection {
        JsonRejection::JsonDataError(_) => {
            ApiError::BadRequest("Invalid JSON format in request body".to_string())
        }
        JsonRejection::JsonSyntaxError(_) => {
            ApiError::BadRequest("Malformed JSON in request body".to_string())
        }
        JsonRejection::MissingJsonContentType(_) => {
            ApiError::BadRequest("Request must have Content-Type: application/json".to_string())
        }
        JsonRejection::BytesRejection(_) => {
            ApiError::BadRequest("Invalid request body".to_string())
        }
        _ => ApiError::BadRequest("Invalid request format".to_string()),
    }
}
