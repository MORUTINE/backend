use axum::Router;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

use crate::bootstrap::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: AppConfig,
}

pub async fn init_state(config: &AppConfig) -> AppState {
    let db_url = config.database.to_postgres_url();

    let mut opt = ConnectOptions::new(db_url.clone());
    opt.max_connections(10) // FIXME: 세팅값은 나중에 최적화
        .min_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(30))
        .max_lifetime(Duration::from_secs(600))
        .sqlx_logging(true);

    let db = Database::connect(opt)
        .await
        .expect("DB 연결에 실패 했습니다.");

    AppState {
        db,
        config: config.clone(),
    }
}

pub async fn run_server(app: Router, port: u16) {
    let addr = format!("0.0.0.0:{port}");

    tracing::info!("서버가 {addr} 에서 실행 중");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
