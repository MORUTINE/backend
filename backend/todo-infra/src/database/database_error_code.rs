use sea_orm::DbErr;
use thiserror::Error;
use common::constant::status::{CONFLICT, INTERNAL_SERVER_ERROR, NOT_FOUND};
use common::error::{ErrorCode, ErrorReason};

#[derive(Debug, Error)]
pub enum DatabaseErrorCode {
    #[error("데이터베이스에 연결할 수 없음")]
    ConnectionError(#[source] DbErr),
    #[error("쿼리 실행 중 오류 발생")]
    QueryError(#[source] DbErr),
    #[error("이미 존재하는 값")]
    UniqueViolation(#[source] DbErr),
    #[error("데이터를 찾을 수 없음")]
    NotFound,
}

impl ErrorCode for DatabaseErrorCode {
    fn reason(&self) -> ErrorReason {
        match self {
            DatabaseErrorCode::ConnectionError(_) => ErrorReason {
                status: INTERNAL_SERVER_ERROR,
                code: "DB_500_1",
                message: "데이터베이스에 연결할 수 없습니다.",
            },
            DatabaseErrorCode::QueryError(_) => ErrorReason {
                status: INTERNAL_SERVER_ERROR,
                code: "DB_500_2",
                message: "쿼리 실행 중 오류가 발생했습니다.",
            },
            DatabaseErrorCode::UniqueViolation(_) => ErrorReason {
                status: CONFLICT,
                code: "DB_409_1",
                message: "이미 존재하는 값입니다.",
            },
            DatabaseErrorCode::NotFound => ErrorReason {
                status: NOT_FOUND,
                code: "DB_404_1",
                message: "데이터를 찾을 수 없습니다.",
            },
        }
    }
}
