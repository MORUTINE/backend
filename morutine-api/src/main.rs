mod bootstrap;
mod common;
mod domain;
mod routes;

use crate::bootstrap::AppState;
use crate::routes::create_app_router;
use bootstrap::{init_logger, load_config, run_server};

#[tokio::main]
async fn main() {
    let config = load_config();
    init_logger(&config);

    let state = AppState::new(&config).await;

    let router = create_app_router(&config).with_state(state);
    run_server(router, config.server.port).await;
}
