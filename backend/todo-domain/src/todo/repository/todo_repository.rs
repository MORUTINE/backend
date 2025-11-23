use crate::todo::models::todo::Todo;
use crate::todo::models::todo_item::TodoItem;
use chrono::NaiveDate;

pub trait TodoRepository {
    fn find_todo_by_user_and_date(
        &self,
        user_id: i64,
        date: NaiveDate,
    ) -> Result<Option<Todo>, anyhow::Error>;

    fn insert_todo(&self, todo: &Todo) -> Result<Todo, anyhow::Error>;

    fn update_todo(&self, todo: &Todo) -> Result<Todo, anyhow::Error>;

    /// item
    fn find_item_by_id(&self, item_id: i64) -> Result<Option<TodoItem>, anyhow::Error>;

    fn find_items_by_todo_id(&self, todo_id: i64) -> Result<Vec<TodoItem>, anyhow::Error>;

    fn insert_item(&self, item: &TodoItem, parent_todo_id: i64) -> Result<TodoItem, anyhow::Error>;

    fn update_item(&self, item: &TodoItem) -> Result<TodoItem, anyhow::Error>;

    fn delete_item(&self, item_id: i64) -> Result<(), anyhow::Error>;
}
