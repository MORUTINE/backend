use crate::bootstrap::config::AppConfig;
use crate::bootstrap::{AppState, build_cors};
use crate::domain::system::routes::system_routes;
use axum::Router;

pub fn create_app_router(config: &AppConfig) -> Router<AppState> {
    Router::new()
        .nest("/system", system_routes())
        .layer(build_cors(config))
}
