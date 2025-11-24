use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::common::error::app_error::AppError;
use crate::common::error::database_error_wrapper::DatabaseApiError;
use common::error::{CommonErrorCode, ErrorCode};
use infra::database::database_error_code::DatabaseErrorCode;

// AppError<E> -> HTTP Response
impl<E: ErrorCode> IntoResponse for AppError<E> {
    fn into_response(self) -> Response {
        let reason = self.code.reason();

        let body = Json(json!({
            "code": reason.code,
            "message": reason.message,
            "data": {}
        }));

        (StatusCode::from_u16(reason.status).unwrap(), body).into_response()
    }
}

// PostgresError -> HTTP Response
impl IntoResponse for DatabaseApiError {
    fn into_response(self) -> Response {
        let app_error: AppError<CommonErrorCode> = match self.0 {
            DatabaseErrorCode::UniqueViolation(_) => AppError::new(CommonErrorCode::Conflict),

            DatabaseErrorCode::NotFound => AppError::new(CommonErrorCode::NotFound),

            _ => AppError::new(CommonErrorCode::InternalServerError),
        };

        app_error.into_response()
    }
}
