use crate::todo::models::todo_item::TodoItem;
use crate::todo::todo_error_code::TodoErrorCode;
use crate::todo::{
    EmptyContent, ItemNotFound, MaxItemLimit, PastDateNotAllowed, StateChangeNotAllowed,
};
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Clone)]
pub struct Todo {
    id: Option<i64>,
    user_id: i64,
    date: NaiveDate,
    items: Vec<TodoItem>,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

/// Infrastructure에서 DB entity → Domain model 변환 시 사용 (id 세팅)
pub struct TodoFromEntity {
    pub id: i64,
    pub user_id: i64,
    pub date: NaiveDate,
    pub items: Vec<TodoItem>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(user_id: i64, date: NaiveDate) -> Result<Self, TodoErrorCode> {
        let today = Utc::now().date_naive();

        if date < today {
            return Err(PastDateNotAllowed);
        }

        Ok(Todo {
            id: None,
            user_id,
            date,
            items: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
        })
    }

    /// DB에서 불러온 데이터를 Domain model로 변환
    pub fn from_entity(entity: TodoFromEntity) -> Self {
        Self {
            id: Some(entity.id),
            user_id: entity.user_id,
            date: entity.date,
            items: entity.items,
            created_at: entity.created_at,
            modified_at: entity.modified_at,
        }
    }

    pub fn add_item(&mut self, content: &str) -> Result<(), TodoErrorCode> {
        self.validate_creatable()?;

        if self.items.len() >= 3 {
            return Err(MaxItemLimit);
        }

        let item = TodoItem::new(self.user_id, content)?;
        self.items.push(item);
        self.modified_at = Utc::now();
        Ok(())
    }

    pub fn update_item_content(
        &mut self,
        item_id: i64,
        new_content: &str,
    ) -> Result<(), TodoErrorCode> {
        self.validate_editable()?;

        if new_content.trim().is_empty() {
            return Err(EmptyContent);
        }

        let item = self.find_mut_item(item_id)?;
        item.update_content(new_content);

        self.modified_at = Utc::now();
        Ok(())
    }

    pub fn complete_item(&mut self, item_id: i64) -> Result<(), TodoErrorCode> {
        self.validate_state_modifiable()?;
        let item = self.find_mut_item(item_id)?;
        item.completed();
        self.modified_at = Utc::now();
        Ok(())
    }

    pub fn alter_item(&mut self, item_id: i64, content: &str) -> Result<(), TodoErrorCode> {
        self.validate_state_modifiable()?;
        let item = self.find_mut_item(item_id)?;
        item.altered(content);
        self.modified_at = Utc::now();
        Ok(())
    }

    pub fn fail_item(&mut self, item_id: i64) -> Result<(), TodoErrorCode> {
        self.validate_state_modifiable()?;
        let item = self.find_mut_item(item_id)?;
        item.failed();
        self.modified_at = Utc::now();
        Ok(())
    }

    /// 오늘이나 미래의 할 일만 추가할 수 있다.
    fn validate_creatable(&self) -> Result<(), TodoErrorCode> {
        let today = Utc::now().date_naive();

        if self.date < today {
            Err(PastDateNotAllowed)
        } else {
            Ok(())
        }
    }

    /// 당일이 되기 전에는 얼마든지 수정/삭제 가능하다.
    fn validate_editable(&self) -> Result<(), TodoErrorCode> {
        let today = Utc::now().date_naive();
        if self.date <= today {
            Err(PastDateNotAllowed)
        } else {
            Ok(())
        }
    }

    /// 할 일의 상태 변경은 당일부터 가능하다.
    fn validate_state_modifiable(&self) -> Result<(), TodoErrorCode> {
        let today = Utc::now().date_naive();
        if self.date > today {
            Err(StateChangeNotAllowed)
        } else {
            Ok(())
        }
    }

    fn find_mut_item(&mut self, item_id: i64) -> Result<&mut TodoItem, TodoErrorCode> {
        self.items
            .iter_mut()
            .find(|item| item.id() == Some(item_id))
            .ok_or(ItemNotFound)
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
    use crate::todo::models::todo_item::TodoItemForTest;
    use crate::todo::models::todo_item_status::TodoItemStatus;
    use TodoErrorCode::MaxItemLimit;
    use TodoItemStatus::{Altered, Completed, Failed};

    fn past_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()
    }

    fn future_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2100, 1, 1).unwrap()
    }

    #[test]
    fn new_todo_success() {
        let date = future_date();
        let todo = Todo::new(818, date).unwrap();

        assert_eq!(todo.user_id(), 818);
        assert_eq!(todo.date(), date);
        assert!(todo.items().is_empty());
        assert!(todo.id().is_none());
    }

    #[test]
    fn new_todo_should_fail_for_past_date() {
        let result = Todo::new(1, past_date());
        assert!(matches!(result, Err(PastDateNotAllowed)));
    }

    #[test]
    fn add_item_success() {
        let date = future_date();
        let mut todo = Todo::new(1, date).unwrap();

        todo.add_item("데드 100kg 10회 10세트").unwrap();
        let past_time = todo.modified_at();

        assert_eq!(todo.items().len(), 1);
        assert!(todo.modified_at() >= past_time);
    }

    #[test]
    fn add_item_should_fail_on_past_todo() {
        let mut todo = Todo {
            id: None,
            user_id: 1,
            date: past_date(),
            items: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
        };

        let result = todo.add_item("벤치 80kg 10회 10세트");
        assert!(matches!(result, Err(PastDateNotAllowed)));
    }

    #[test]
    fn add_item_should_fail_when_exceeds_limit() {
        let date = future_date();
        let mut todo = Todo::new(1, date).unwrap();

        todo.add_item("벤치 80kg 10회 10세트").unwrap();
        todo.add_item("스쿼트 100kg 10회 10세트").unwrap();
        todo.add_item("데드 100kg 10회 10세트").unwrap();

        let result = todo.add_item("OHP 40kg 10회 10세트");

        assert!(matches!(result, Err(MaxItemLimit)));
    }

    #[test]
    fn complete_item_success() {
        let date = Utc::now().date_naive();
        let mut todo = Todo::new(1, date).unwrap();

        // 원래 add_item 메서드 써서 넣어야 되는데, 테스트용으로 직접 아이템을 추가
        let item = TodoItem::for_test(TodoItemForTest {
            id: 10,
            // todo_id도 테스트용으로 임시 더미값
            todo_id: 1,
            content: "벤치프레스".to_string(),
            status: TodoItemStatus::Pending,
        });

        todo.items.push(item);

        todo.complete_item(10).unwrap();

        assert_eq!(todo.items[0].status(), Completed);
    }

    #[test]
    fn complete_item_should_fail_for_not_found() {
        let date = Utc::now().date_naive();
        let mut todo = Todo::new(1, date).unwrap();

        let result = todo.complete_item(999);

        assert!(matches!(result, Err(ItemNotFound)));
    }

    #[test]
    fn alter_item_success() {
        let date = Utc::now().date_naive();
        let mut todo = Todo::new(1, date).unwrap();

        let item = TodoItem::for_test(TodoItemForTest {
            id: 10,
            todo_id: 1,
            content: "벤치프레스".to_string(),
            status: TodoItemStatus::Pending,
        });

        todo.items.push(item);

        todo.alter_item(5, "사람 많아서 스쿼트로 변경").unwrap();

        assert_eq!(todo.items[0].status(), Altered);
        assert_eq!(
            todo.items[0].altered_plan().unwrap().as_str(),
            "사람 많아서 스쿼트로 변경"
        );
    }

    #[test]
    fn fail_item_success() {
        let date = Utc::now().date_naive();
        let mut todo = Todo::new(1, date).unwrap();

        let item = TodoItem::for_test(TodoItemForTest {
            id: 3,
            todo_id: 1,
            content: "벤치프레스".to_string(),
            status: TodoItemStatus::Pending,
        });

        todo.items.push(item);

        todo.fail_item(3).unwrap();

        assert_eq!(todo.items[0].status(), Failed);
    }

    #[test]
    fn update_item_content_should_succeed_until_that_date() {
        let mut todo = Todo::new(1, future_date()).unwrap();

        let item = TodoItem::for_test(TodoItemForTest {
            id: 10,
            todo_id: 9,
            content: "벤치프레스".to_string(),
            status: TodoItemStatus::Pending,
        });

        todo.items.push(item);

        let res = todo.update_item_content(10, "스쿼트");
        assert!(res.is_ok());
        assert_eq!(todo.items[0].content(), "스쿼트");
    }

    #[test]
    fn update_item_content_should_fail_for_today_or_past_todo() {
        let mut todo = Todo {
            id: None,
            user_id: 1,
            date: past_date(),
            items: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
        };

        let item = TodoItem::for_test(TodoItemForTest {
            id: 10,
            todo_id: 1,
            content: "벤치 80kg 10회 10세트".to_string(),
            status: TodoItemStatus::Pending,
        });

        todo.items.push(item);

        let res = todo.update_item_content(5, "데드 100kg 10회 10세트");
        assert!(matches!(res, Err(PastDateNotAllowed)));
    }
}
