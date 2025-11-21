use super::handlers::health_check;
use crate::bootstrap::AppState;
use axum::{Router, routing::options};

pub fn system_routes() -> Router<AppState> {
    Router::new().route("/health", options(health_check))
}
