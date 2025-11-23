use infra::database::error::DatabaseError;

#[derive(Debug)]
pub struct DatabaseApiError(pub DatabaseError);
