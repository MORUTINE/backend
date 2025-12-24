use async_trait::async_trait;
use domain::todo::models::todo::TodoFromEntity;

#[async_trait]
pub trait TodoQueryRepository: Send + Sync {
    async fn find_monthly_todos(
        &self,
        user_id: i64,
        year: u32,
        month: u32,
    ) -> Result<Vec<TodoFromEntity>, anyhow::Error>;
}
