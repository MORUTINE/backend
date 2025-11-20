mod bootstrap;
mod common;

use bootstrap::{load_config, init_state};
use crate::bootstrap::init_logger;

#[tokio::main]
async fn main() {
    let config = load_config();
    init_logger(&config);

    let state = init_state(&config).await;
    tracing::info!("DB 연결 성공");
}