use axum::extract::{rejection::PathRejection, FromRequestParts, Path};
use axum::http::request::Parts;
use serde::de::DeserializeOwned;
use std::future::Future;
use tracing::debug;

use crate::common::error::ApiError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedPath<T>(pub T);

impl<T> std::ops::Deref for ValidatedPath<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for ValidatedPath<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, S> FromRequestParts<S> for ValidatedPath<T>
where
    T: DeserializeOwned + Send + Sync,
    S: Send + Sync,
{
    type Rejection = ApiError;

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let Path(value) =
                Path::<T>::from_request_parts(parts, state)
                    .await
                    .map_err(|rejection| {
                        debug!("Path parsing error: {:?}", rejection);
                        map_path_rejection_to_user_error(rejection)
                    })?;

            Ok(ValidatedPath(value))
        }
    }
}

fn map_path_rejection_to_user_error(rejection: PathRejection) -> ApiError {
    use axum::extract::rejection::*;

    match rejection {
        PathRejection::FailedToDeserializePathParams(inner) => {
            debug!("Path deserialization failed: {:?}", inner);
            ApiError::BadRequest("Invalid path parameter format".to_string())
        }
        PathRejection::MissingPathParams(inner) => {
            debug!("Missing path params: {:?}", inner);
            ApiError::BadRequest("Missing required path parameters".to_string())
        }
        _ => {
            debug!("Unknown path rejection: {:?}", rejection);
            ApiError::BadRequest("Invalid path parameters".to_string())
        }
    }
}
