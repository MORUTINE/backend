use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

pub async fn health_check() -> impl IntoResponse {
    Json(json!({"status": "OK"}))
}
