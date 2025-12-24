use crate::domain::todo::application::dto::result::MonthlyTodosResult;
use crate::domain::todo::{TodoDto, TodoItemDto};
use serde::Serialize;

/// GET /todos 응답 (월별)
#[derive(Debug, Serialize)]
pub struct MonthlyTodoResponse {
    pub days: Vec<DailyTodoResponse>,
}

/// 하루치 할 일
#[derive(Debug, Serialize)]
pub struct DailyTodoResponse {
    pub date: String, // "YYYY-MM-DD"
    pub todos: Vec<TodoItemResponse>,
}

/// 개별 할 일 항목
#[derive(Debug, Serialize)]
pub struct TodoItemResponse {
    pub id: i64,
    pub content: String,
    pub status: String, // "PENDING", "COMPLETED", "ALTERED", "FAILED"
    pub altered_content: Option<String>,
    pub image_url: Option<String>,
}

impl From<MonthlyTodosResult> for MonthlyTodoResponse {
    fn from(result: MonthlyTodosResult) -> Self {
        let days = result
            .todos
            .into_iter()
            .map(DailyTodoResponse::from)
            .collect();
        Self { days }
    }
}

impl From<TodoDto> for DailyTodoResponse {
    fn from(todo: TodoDto) -> Self {
        Self {
            date: todo.date.to_string(),
            todos: todo.items.into_iter().map(TodoItemResponse::from).collect(),
        }
    }
}

impl From<TodoItemDto> for TodoItemResponse {
    fn from(item: TodoItemDto) -> Self {
        Self {
            id: item.id,
            content: item.content,
            status: item.status,
            altered_content: item.altered_content,
            image_url: item.image_url,
        }
    }
}
