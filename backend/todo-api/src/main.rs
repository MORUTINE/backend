mod bootstrap;
mod common;
mod domain;
mod routes;

use crate::routes::create_app_router;
use bootstrap::{init_logger, init_state, load_config, run_server};

#[tokio::main]
async fn main() {
    let config = load_config();
    init_logger(&config);

    let state = init_state(&config).await;

    let router = create_app_router().with_state(state);

    run_server(router, config.server.port).await;
}
