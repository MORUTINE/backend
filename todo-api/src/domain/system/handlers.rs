use crate::common::response::api_response::ApiResponse;
use axum::response::IntoResponse;

pub async fn health_check() -> impl IntoResponse {
    ApiResponse::success("OK")
}
