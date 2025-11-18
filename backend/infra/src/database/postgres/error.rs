#[derive(Debug)]
pub enum DbError {
    ConnectionError,
    Timeout,
    UniqueViolation,
    NotFound,
}