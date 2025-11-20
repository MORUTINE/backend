use axum::Router;
use sea_orm::{Database, DatabaseConnection};

use crate::bootstrap::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: AppConfig,
}

pub async fn init_state(config: &AppConfig) -> AppState {
    let db_url = config.database.to_postgres_url();

    let db = Database::connect(&db_url)
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
