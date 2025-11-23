use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("database connection error")]
    ConnectionError,

    #[error("query execution error")]
    QueryError,

    #[error("unique constraint violated")]
    UniqueViolation,

    #[error("record not found")]
    NotFound,
}
