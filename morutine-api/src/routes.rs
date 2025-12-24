use crate::bootstrap::config::AppConfig;
use crate::bootstrap::{
    AppState, build_compression_layer, build_concurrency_limit_layer, build_cors,
};
use crate::common::auth::mock_auth_middleware;
use crate::domain::system::routes::system_routes;
use crate::domain::todo::todo_routes::todo_routes;
use axum::Router;

pub fn create_app_router(config: &AppConfig) -> Router<AppState> {
    Router::new().nest(
        "/api/v1",
        Router::new()
            .nest("/system", system_routes())
            .nest(
                "/todos",
                todo_routes().layer(axum::middleware::from_fn(mock_auth_middleware)),
            )
            .layer(build_compression_layer())
            .layer(build_concurrency_limit_layer())
            .layer(build_cors(config)),
    )
}
