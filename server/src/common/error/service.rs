use sea_orm::DbErr;
use thiserror::Error;

use super::database::DatabaseError;

pub type ServiceResult<T> = Result<T, ServiceError>;

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

impl From<DbErr> for ServiceError {
    fn from(err: DbErr) -> Self {
        ServiceError::DatabaseError(err.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db_err_maps_to_service_error() {
        let db_err = DbErr::RecordNotFound("Record not found".to_string());
        let service_err: ServiceError = db_err.into();
        assert!(matches!(service_err, ServiceError::DatabaseError(_)));
    }
}
