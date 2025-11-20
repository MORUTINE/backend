mod bootstrap;
mod common;

use crate::bootstrap::init_logger;
use bootstrap::{init_state, load_config};

#[tokio::main]
async fn main() {
    let config = load_config();
    init_logger(&config);

    println!("{:#?}", config);

    let state = init_state(&config).await;
    tracing::info!("DB 연결 성공");
}
