use crate::todo::error::TodoError::{self, EmptyContent};
use crate::todo::models::todo_item_status::TodoItemStatus;
use TodoItemStatus::{Altered, Completed, Failed, Pending};
use chrono::{DateTime, Utc};
use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct TodoItem {
    id: Option<i64>,
    todo_id: i64,
    content: String,
    status: TodoItemStatus,
    #[builder(default)]
    altered_content: Option<String>,
    #[builder(default)]
    image_url: Option<String>,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

impl TodoItem {
    pub(crate) fn new(todo_id: i64, content: &str) -> Result<Self, TodoError> {
        if content.trim().is_empty() {
            return Err(EmptyContent);
        }

        Ok(Self {
            id: None,
            todo_id,
            content: content.to_string(),
            status: Pending,
            altered_content: None,
            image_url: None,
            created_at: Utc::now(),
            modified_at: Utc::now(),
        })
    }

    pub(crate) fn update_content(&mut self, new_content: &str) {
        self.content = new_content.to_string();
        self.modified_at = Utc::now();
    }

    pub(crate) fn completed(&mut self) {
        self.status = Completed;
        self.modified_at = Utc::now();
    }

    pub(crate) fn altered(&mut self, content: &str) {
        self.status = Altered;
        self.altered_content = Some(content.to_string());
        self.modified_at = Utc::now();
    }

    pub(crate) fn failed(&mut self) {
        self.status = Failed;
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
        self.altered_content.as_ref()
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

    #[test]
    fn new_todo_item_success() {
        let item = TodoItem::new(1, "벤치 프레스 80kg 10회 10세트").unwrap();

        assert_eq!(item.todo_id(), 1);
        assert_eq!(item.content(), "벤치 프레스 80kg 10회 10세트");
        assert_eq!(item.status(), Pending);
        assert!(item.id().is_none());
    }

    #[test]
    fn new_todo_item_empty_content_should_fail() {
        let res = TodoItem::new(1, "   ");
        assert!(matches!(res, Err(EmptyContent)));
    }

    #[test]
    fn update_content_should_change_content_and_modified_at() {
        let mut item = TodoItem::new(1, "벤치 프레스 80kg 10회 10세트").unwrap();
        let past_time = item.modified_at();

        item.update_content("딥스 10회 10세트");

        assert_eq!(item.content(), "딥스 10회 10세트");
        assert!(item.modified_at() > past_time);
    }

    #[test]
    fn complete_should_update_status_and_modified_at() {
        let mut item = TodoItem::new(1, "todo 프로젝트 Rust Axum 백엔드 서버 구축").unwrap();
        let past_time = item.modified_at();

        item.completed();

        assert_eq!(item.status(), Completed);
        assert!(item.modified_at() > past_time);
    }

    #[test]
    fn altered_should_update_status_and_plan() {
        let mut item = TodoItem::new(1, "벤치 프레스 80kg 10회 10세트").unwrap();

        item.altered("오늘 벤치 사람 너무 많아서, 스쿼트 100kg 10회 10세트로 대체");

        assert_eq!(item.status(), Altered);
        assert_eq!(
            item.altered_plan(),
            Some(&"오늘 벤치 사람 너무 많아서, 스쿼트 100kg 10회 10세트로 대체".to_string())
        );
    }

    #[test]
    fn failed_should_update_status() {
        let mut item = TodoItem::new(1, "todo 프로젝트 Svelte 프론트 제작").unwrap();

        item.failed();

        assert_eq!(item.status(), Failed);
    }
}
