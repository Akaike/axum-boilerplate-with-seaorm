use sea_orm::{sqlx, DbErr, RuntimeErr, SqlErr};
use thiserror::Error;

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

            _ if is_unique_constraint_violation(&err) => Conflict,

            DbErr::Exec(RuntimeErr::SqlxError(sqlx_err))
            | DbErr::Query(RuntimeErr::SqlxError(sqlx_err)) => handle_sqlx_error(sqlx_err),

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

fn is_unique_constraint_violation(err: &DbErr) -> bool {
    matches!(err.sql_err(), Some(SqlErr::UniqueConstraintViolation(_)))
}

fn handle_sqlx_error(sqlx_err: &sqlx::Error) -> DatabaseError {
    match sqlx_err {
        sqlx::Error::Database(db_err) => handle_database_error_code(db_err.code().as_deref()),
        _ => DatabaseError::OperationFailed,
    }
}

fn handle_database_error_code(code: Option<&str>) -> DatabaseError {
    match code {
        Some("42P01") => DatabaseError::Internal, // undefined_table
        Some("42703") => DatabaseError::Internal, // undefined_column
        Some("42883") => DatabaseError::Internal, // undefined_function
        _ => DatabaseError::OperationFailed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_not_found_maps_correctly() {
        let db_err = DbErr::RecordNotFound("Record not found".to_string());
        let database_error: DatabaseError = db_err.into();
        assert!(matches!(database_error, DatabaseError::NotFound));
    }
}
