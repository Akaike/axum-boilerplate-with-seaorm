pub mod api;
pub mod database;
pub mod service;

pub use api::{ApiError, ApiResult};
pub use database::DatabaseError;
pub use service::{ServiceError, ServiceResult};

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
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
}
