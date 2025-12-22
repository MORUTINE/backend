use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::common::error::app_error::AppError;
use common::error::{CommonErrorCode, ErrorCode};
use domain::todo::todo_error_code::TodoErrorCode;
use domain::user::user_error_code::UserErrorCode;
use infra::database::database_error_code::DatabaseErrorCode;

// AppError -> HTTP Response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error = self.0;

        // 공통 에러
        if let Some(e) = error.downcast_ref::<CommonErrorCode>() {
            return make_response(e);
        }

        // 각 도메인 영역 에러
        if let Some(e) = error.downcast_ref::<TodoErrorCode>() {
            return make_response(e);
        }

        if let Some(e) = error.downcast_ref::<UserErrorCode>() {
            return make_response(e);
        }

        // DB 에러 (외부 노출 x)
        if let Some(e) = error.downcast_ref::<DatabaseErrorCode>() {
            match e {
                DatabaseErrorCode::UniqueViolation(_) => {
                    return make_response(&CommonErrorCode::Conflict);
                }
                DatabaseErrorCode::NotFound => return make_response(&CommonErrorCode::NotFound),
                _ => {}
            }
        }

        // 그 외 모든 에러 -> 500 서버 에러 & 로깅
        tracing::error!("{:?}", error);
        make_response(&CommonErrorCode::InternalServerError)
    }
}

fn make_response(e: &impl ErrorCode) -> Response {
    let reason = e.reason();

    let body = Json(json!({
        "code": reason.code,
        "message": reason.message,
    }));

    (StatusCode::from_u16(reason.status).unwrap(), body).into_response()
}
