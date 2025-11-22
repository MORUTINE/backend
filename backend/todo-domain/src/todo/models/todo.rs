use crate::todo::error;
use crate::todo::models::todo_item::TodoItem;
use chrono::{DateTime, NaiveDate, Utc};
use derive_builder::Builder;
use error::TodoError;

#[derive(Debug, Clone, Builder)]
pub struct Todo {
    id: Option<i64>,
    user_id: i64,
    date: NaiveDate,
    items: Vec<TodoItem>,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(user_id: i64, date: NaiveDate) -> Self {
        Todo {
            id: None,
            user_id,
            date,
            items: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
        }
    }

    pub fn add_item(&mut self, item: TodoItem) -> Result<(), TodoError> {
        if self.items.len() >= 3 {
            return Err(TodoError::MaxItemLimit);
        }

        self.items.push(item);
        self.modified_at = Utc::now();
        Ok(())
    }

    pub fn id(&self) -> Option<i64> {
        self.id
    }
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
    pub fn date(&self) -> NaiveDate {
        self.date
    }
    pub fn items(&self) -> &Vec<TodoItem> {
        &self.items
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
    use crate::todo::models::todo_item::TodoItem;

    #[test]
    fn new_todo_success() {
        let date = NaiveDate::from_ymd_opt(2025, 11, 22).unwrap();
        let todo = Todo::new(818, date);

        assert_eq!(todo.user_id(), 818);
        assert_eq!(todo.date(), date);
        assert!(todo.items().is_empty());
        assert!(todo.id().is_none());
    }

    #[test]
    fn add_item_success() {
        let date = NaiveDate::from_ymd_opt(2025, 11, 22).unwrap();
        let mut todo = Todo::new(1, date);

        let item = TodoItem::new(1, "데드 100kg 10회 10세트".to_string()).unwrap();
        let prev_time = todo.modified_at();

        todo.add_item(item).unwrap();

        assert_eq!(todo.items().len(), 1);
        assert!(todo.modified_at() > prev_time);
    }

    #[test]
    fn add_item_should_fail_when_exceeds_limit() {
        let date = NaiveDate::from_ymd_opt(2025, 11, 22).unwrap();
        let mut todo = Todo::new(1, date);

        todo.add_item(TodoItem::new(1, "벤치 80kg 10회 10세트".to_string()).unwrap()).unwrap();
        todo.add_item(TodoItem::new(1, "스쿼트 100kg 10회 10세트".to_string()).unwrap()).unwrap();
        todo.add_item(TodoItem::new(1, "데드 100kg 10회 10세트".to_string()).unwrap()).unwrap();

        let result = todo.add_item(TodoItem::new(1, "OHP 40kg 10회 10세트".to_string()).unwrap());

        assert!(matches!(result, Err(TodoError::MaxItemLimit)));
    }
}
