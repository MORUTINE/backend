use crate::database::postgres::todo::todo_query_repository::TodoQueryRepository;
use async_trait::async_trait;
use domain::todo::models::todo::TodoFromEntity;

pub struct TodoQueryRepositoryImpl {
    db: sea_orm::DatabaseConnection,
}

impl TodoQueryRepositoryImpl {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TodoQueryRepository for TodoQueryRepositoryImpl {
    async fn find_monthly_todos(
        &self,
        user_id: i64,
        year: u32,
        month: u32,
    ) -> Result<Vec<TodoFromEntity>, anyhow::Error> {
        // TODO!
        Ok(vec![])
    }
}
