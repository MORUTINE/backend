use crate::todo::models::todo::Todo;
use chrono::NaiveDate;

pub trait TodoRepository {
    fn find_by_user_and_date(
        &self,
        user_id: i64,
        date: NaiveDate,
    ) -> Result<Option<Todo>, anyhow::Error>;
    fn save(&self, todo: &Todo) -> Result<Todo, anyhow::Error>;
}
