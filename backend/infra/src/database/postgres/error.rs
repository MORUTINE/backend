#[derive(Debug)]
pub enum PostgresError {
    ConnectionError,
    Timeout,
    UniqueViolation,
    NotFound,
}