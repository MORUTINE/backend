use crate::todo::error::TodoError;
use crate::todo::models::todo_item_status::TodoItemStatus;
use chrono::{DateTime, Utc};
use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct TodoItem {
    id: Option<i64>,
    todo_id: i64,
    content: String,
    status: TodoItemStatus,
    altered_plan: Option<String>,
    image_url: Option<String>,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

impl TodoItem {
    pub fn new(todo_id: i64, content: String) -> Result<Self, TodoError> {
        if content.trim().is_empty() {
            return Err(TodoError::EmptyContent);
        }

        Ok(Self {
            id: None,
            todo_id,
            content,
            status: TodoItemStatus::Pending,
            altered_plan: None,
            image_url: None,
            created_at: Utc::now(),
            modified_at: Utc::now(),
        })
    }

    pub fn completed(&mut self) {
        self.status = TodoItemStatus::Completed;
        self.modified_at = Utc::now();
    }

    pub fn altered(&mut self, content: String) {
        self.status = TodoItemStatus::Altered;
        self.altered_plan = Some(content);
        self.modified_at = Utc::now();
    }

    pub fn failed(&mut self) {
        self.status = TodoItemStatus::Failed;
        self.modified_at = Utc::now();
    }

    pub fn id(&self) -> Option<i64> {
        self.id
    }
    pub fn todo_id(&self) -> i64 {
        self.todo_id
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn status(&self) -> TodoItemStatus {
        self.status
    }
    pub fn altered_plan(&self) -> Option<&String> {
        self.altered_plan.as_ref()
    }
    pub fn image_url(&self) -> Option<&String> {
        self.image_url.as_ref()
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn modified_at(&self) -> DateTime<Utc> {
        self.modified_at
    }
}
