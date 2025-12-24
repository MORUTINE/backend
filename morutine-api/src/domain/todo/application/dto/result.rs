use chrono::NaiveDate;
use domain::todo::models::todo::TodoFromEntity;
use domain::todo::models::todo_item::TodoItemFromEntity;

#[derive(Debug)]
pub struct MonthlyTodosResult {
    pub todos: Vec<TodoDto>,
}

#[derive(Debug)]
pub struct TodoDto {
    pub id: i64,
    pub date: NaiveDate,
    pub items: Vec<TodoItemDto>,
}

#[derive(Debug)]
pub struct TodoItemDto {
    pub id: i64,
    pub content: String,
    pub status: String,
    pub altered_content: Option<String>,
    pub image_url: Option<String>,
}

impl From<TodoFromEntity> for TodoDto {
    fn from(model: TodoFromEntity) -> Self {
        Self {
            id: model.id,
            date: model.date,
            items: model.items.into_iter().map(TodoItemDto::from).collect(),
        }
    }
}

impl From<TodoItemFromEntity> for TodoItemDto {
    fn from(model: TodoItemFromEntity) -> Self {
        Self {
            id: model.id,
            content: model.content,
            status: model.status.to_string(),
            altered_content: model.altered_content,
            image_url: model.image_url,
        }
    }
}
