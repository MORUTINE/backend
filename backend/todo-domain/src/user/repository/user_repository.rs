use crate::user::models::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, anyhow::Error>;
    async fn find_by_nickname(&self, nickname: &str) -> Result<Option<User>, anyhow::Error>;

    async fn insert(&self, user: &mut User) -> Result<User, anyhow::Error>;
    async fn update(&self, user: &User) -> Result<User, anyhow::Error>;
    async fn delete(&self, id: i64) -> Result<(), anyhow::Error>;
}
