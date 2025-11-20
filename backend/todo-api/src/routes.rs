use crate::bootstrap::AppState;
use crate::domain::system::routes::system_routes;
use axum::Router;

pub fn create_app_router() -> Router<AppState> {
    Router::new().nest("/system", system_routes())
}
