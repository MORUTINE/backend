use crate::todo::models::todo_item::TodoItem;

pub trait TodoItemRepository {
    fn save(&self, item: &TodoItem, parent_todo_id: i64) -> Result<TodoItem, anyhow::Error>;
}
