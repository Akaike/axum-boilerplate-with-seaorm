mod api;
mod auth;
mod db;

pub use api::ApiError;
pub use auth::AuthError;
pub use db::DbError;

use axum::response::{IntoResponse, Response};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Api(#[from] ApiError),

    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Database(#[from] DbError),
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
