use crate::todo::models::todo::Todo;
use crate::todo::models::todo_item::TodoItem;
use async_trait::async_trait;
use chrono::NaiveDate;

#[async_trait]
pub trait TodoRepository {
    async fn find_todo_by_user_and_date(
        &self,
        user_id: i64,
        date: NaiveDate,
    ) -> Result<Option<Todo>, anyhow::Error>;

    async fn insert_todo(&self, todo: &Todo) -> Result<Todo, anyhow::Error>;

    async fn update_todo(&self, todo: &Todo) -> Result<Todo, anyhow::Error>;

    /// item
    async fn find_item_by_id(&self, item_id: i64) -> Result<Option<TodoItem>, anyhow::Error>;

    async fn find_items_by_todo_id(&self, todo_id: i64) -> Result<Vec<TodoItem>, anyhow::Error>;

    async fn insert_item(
        &self,
        item: &TodoItem,
        parent_todo_id: i64,
    ) -> Result<TodoItem, anyhow::Error>;

    async fn update_item(&self, item: &TodoItem) -> Result<TodoItem, anyhow::Error>;

    async fn delete_item(&self, item_id: i64) -> Result<(), anyhow::Error>;
}
