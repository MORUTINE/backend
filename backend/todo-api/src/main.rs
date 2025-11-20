mod bootstrap;
mod common;

use bootstrap::{load_config, init_state};

#[tokio::main]
async fn main() {
    let config = load_config();

    let state = init_state(&config).await;

}