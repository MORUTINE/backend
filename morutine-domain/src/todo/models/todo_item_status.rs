use crate::todo::todo_error_code::TodoErrorCode;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TodoItemStatus {
    Pending,   // 아직 수행 전
    Completed, // 정상 완료
    Altered,   // 대체 업무 수행
    Failed,    // 실패
}

impl TodoItemStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TodoItemStatus::Pending => "PENDING",
            TodoItemStatus::Completed => "COMPLETED",
            TodoItemStatus::Altered => "ALTERED",
            TodoItemStatus::Failed => "FAILED",
        }
    }
}

impl fmt::Display for TodoItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for TodoItemStatus {
    type Error = TodoErrorCode;

    fn try_from(value: &str) -> Result<Self, TodoErrorCode> {
        match value {
            "PENDING" => Ok(TodoItemStatus::Pending),
            "COMPLETED" => Ok(TodoItemStatus::Completed),
            "ALTERED" => Ok(TodoItemStatus::Altered),
            "FAILED" => Ok(TodoItemStatus::Failed),
            _ => Err(TodoErrorCode::InvalidStatus),
        }
    }
}
