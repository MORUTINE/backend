use infra::database::postgres::PostgresError;

#[derive(Debug)]
pub struct PostgresApiError(pub PostgresError);
