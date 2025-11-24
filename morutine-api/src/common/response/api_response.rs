use axum::{Json, http::StatusCode, response::IntoResponse};
use common::constant::status::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub code: &'static str,
    pub message: Option<&'static str>,
    pub data: Option<T>,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::OK);
        let body = Json(self);

        (status, body).into_response()
    }
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            status: OK,
            code: "OK",
            message: None,
            data: Some(data),
        }
    }

    pub fn no_content() -> Self {
        ApiResponse {
            status: NO_CONTENT,
            code: "NO_CONTENT",
            message: None,
            data: None,
        }
    }
}
