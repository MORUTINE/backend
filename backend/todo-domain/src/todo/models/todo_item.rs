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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::models::todo_item_status::TodoItemStatus;

    #[test]
    fn new_todo_item_success() {
        let item = TodoItem::new(1, "벤치 프레스 80kg 10회 10세트".to_string()).unwrap();

        assert_eq!(item.todo_id(), 1);
        assert_eq!(item.content(), "벤치 프레스 80kg 10회 10세트");
        assert_eq!(item.status(), TodoItemStatus::Pending);
        assert!(item.id().is_none());
    }

    #[test]
    fn new_todo_item_empty_content_should_fail() {
        let res = TodoItem::new(1, "   ".to_string());
        assert!(matches!(res, Err(TodoError::EmptyContent)));
    }

    #[test]
    fn complete_should_update_status_and_modified_at() {
        let mut item = TodoItem::new(1, "todo 프로젝트 Rust Axum 백엔드 서버 구축".to_string()).unwrap();
        let old_time = item.modified_at();

        item.completed();

        assert_eq!(item.status(), TodoItemStatus::Completed);
        assert!(item.modified_at() > old_time);
    }

    #[test]
    fn altered_should_update_status_and_plan() {
        let mut item = TodoItem::new(1, "벤치 프레스 80kg 10회 10세트".to_string()).unwrap();

        item.altered("오늘 벤치 사람 너무 많아서, 스쿼트 100kg 10회 10세트로 대체".to_string());

        assert_eq!(item.status(), TodoItemStatus::Altered);
        assert_eq!(item.altered_plan(), Some(&"오늘 벤치 사람 너무 많아서, 스쿼트 100kg 10회 10세트로 대체".to_string()));
    }

    #[test]
    fn failed_should_update_status() {
        let mut item = TodoItem::new(1, "todo 프로젝트 Svelte 프론트 제작".to_string()).unwrap();

        item.failed();

        assert_eq!(item.status(), TodoItemStatus::Failed);
    }
}
