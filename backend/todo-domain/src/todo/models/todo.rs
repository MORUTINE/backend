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
