use infra::database::database_error_code::DatabaseErrorCode;

#[derive(Debug)]
pub struct DatabaseApiError(pub DatabaseErrorCode);
