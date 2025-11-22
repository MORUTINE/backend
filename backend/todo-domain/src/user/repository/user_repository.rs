use crate::user::models::user::User;

pub trait UserRepository {
    fn find_by_id(&self, id: i64) -> Result<Option<User>, anyhow::Error>;
    fn find_by_nickname(&self, nickname: &str) -> Result<Option<User>, anyhow::Error>;
    fn save(&self, user: &mut User) -> Result<(), anyhow::Error>;
}
