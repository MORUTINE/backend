use super::handlers::health_check;
use crate::bootstrap::AppState;
use axum::{Router, routing::get};

pub fn system_routes() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}
