use crate::domain::todo::TodoDto;
use crate::domain::todo::application::dto::result::MonthlyTodosResult;
use domain::todo::repository::todo_repository::TodoRepository;
use infra::database::postgres::todo::todo_query_repository::TodoQueryRepository;
use std::sync::Arc;

pub struct TodoService {
    todo_repository: Arc<dyn TodoRepository>,
    todo_query_repository: Arc<dyn TodoQueryRepository>,
}

impl TodoService {
    pub fn new(
        todo_repository: Arc<dyn TodoRepository>,
        todo_query_repository: Arc<dyn TodoQueryRepository>,
    ) -> Self {
        Self {
            todo_repository,
            todo_query_repository,
        }
    }

    pub async fn get_monthly_todos(
        &self,
        user_id: i64,
        year: u32,
        month: u32,
    ) -> anyhow::Result<MonthlyTodosResult> {
        let todos_data = self
            .todo_query_repository
            .find_monthly_todos(user_id, year, month)
            .await?;

        Ok(MonthlyTodosResult {
            todos: todos_data.into_iter().map(TodoDto::from).collect(),
        })
    }
}
