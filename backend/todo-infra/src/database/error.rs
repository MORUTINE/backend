use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("데이터베이스에 연결할 수 없음")]
    ConnectionError(#[source] DbErr),
    #[error("쿼리 실행 중 오류 발생")]
    QueryError(#[source] DbErr),
    #[error("이미 존재하는 값")]
    UniqueViolation(#[source] DbErr),
    #[error("데이터를 찾을 수 없음")]
    NotFound,
}
