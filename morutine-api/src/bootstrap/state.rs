use crate::bootstrap::config::AppConfig;
use crate::domain::todo::application::todo_service::TodoService;
use axum::Router;
use domain::todo::repository::todo_repository::TodoRepository;
use infra::database::postgres::todo::todo_query_repository::TodoQueryRepository;
use infra::database::postgres::todo::todo_query_repository_impl::TodoQueryRepositoryImpl;
use infra::database::postgres::todo::todo_repository_impl::TodoRepositoryImpl;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct AppState {
    pub todo_service: Arc<TodoService>,
}

impl AppState {
    pub async fn new(config: &AppConfig) -> Self {
        let db = Self::init_db(config).await;

        let todo_service = Self::create_todo_service(&db);

        Self { todo_service }
    }

    async fn init_db(config: &AppConfig) -> DatabaseConnection {
        let db_url = config.database.to_postgres_url();

        let mut opt = ConnectOptions::new(db_url.clone());
        opt.max_connections(10) // FIXME: 세팅값은 나중에 최적화
            .min_connections(5)
            .connect_timeout(Duration::from_secs(5))
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(30))
            .max_lifetime(Duration::from_secs(600))
            .sqlx_logging(true);

        Database::connect(opt)
            .await
            .expect("DB 연결에 실패 했습니다.")
    }

    fn create_todo_service(db: &DatabaseConnection) -> Arc<TodoService> {
        let todo_repository: Arc<dyn TodoRepository> =
            Arc::new(TodoRepositoryImpl::new(db.clone()));

        let todo_query_repository: Arc<dyn TodoQueryRepository> =
            Arc::new(TodoQueryRepositoryImpl::new(db.clone()));

        Arc::new(TodoService::new(todo_repository, todo_query_repository))
    }
}

pub async fn run_server(app: Router, port: u16) {
    let addr = format!("0.0.0.0:{port}");
    tracing::info!("{addr} 에서 서버 실행 중");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
