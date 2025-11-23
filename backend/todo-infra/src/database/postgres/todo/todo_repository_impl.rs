use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use super::todo_mapper::TodoMapper;
use super::{todo_entity, todo_item_entity};
use crate::database::error::DatabaseError;
use domain::todo::models::{todo::Todo, todo_item::TodoItem};
use domain::todo::repository::todo_repository::TodoRepository;

pub struct TodoRepositoryImpl {
    pub db: sea_orm::DatabaseConnection,
}

impl TodoRepositoryImpl {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn find_todo_by_user_and_date(
        &self,
        user_id: i64,
        date: chrono::NaiveDate,
    ) -> Result<Option<Todo>, anyhow::Error> {
        let todo = todo_entity::Entity::find()
            .filter(todo_entity::Column::UserId.eq(user_id))
            .filter(todo_entity::Column::Date.eq(date))
            .one(&self.db)
            .await
            .map_err(DatabaseError::QueryError)?;

        let Some(todo) = todo else {
            return Ok(None);
        };

        let items = todo_item_entity::Entity::find()
            .filter(todo_item_entity::Column::TodoId.eq(todo.id))
            .all(&self.db)
            .await
            .map_err(DatabaseError::QueryError)?;

        let mapped = TodoMapper::map_entity_to_todo(todo, items)?;

        Ok(Some(mapped))
    }

    async fn insert_todo(&self, todo: &Todo) -> Result<Todo, anyhow::Error> {
        let active = TodoMapper::map_todo_to_entity(todo);

        let saved = active
            .insert(&self.db)
            .await
            .map_err(DatabaseError::QueryError)?;

        Ok(TodoMapper::map_entity_to_todo(saved, vec![])?)
    }

    async fn update_todo(&self, todo: &Todo) -> Result<Todo, anyhow::Error> {
        let active = TodoMapper::map_todo_to_entity(todo);

        let saved = active
            .update(&self.db)
            .await
            .map_err(DatabaseError::QueryError)?;

        let items = todo_item_entity::Entity::find()
            .filter(todo_item_entity::Column::TodoId.eq(saved.id))
            .all(&self.db)
            .await
            .map_err(DatabaseError::QueryError)?;

        Ok(TodoMapper::map_entity_to_todo(saved, items)?)
    }

    async fn find_item_by_id(&self, item_id: i64) -> Result<Option<TodoItem>, anyhow::Error> {
        let model = todo_item_entity::Entity::find_by_id(item_id)
            .one(&self.db)
            .await
            .map_err(DatabaseError::QueryError)?;

        Ok(model
            .map(|m| TodoMapper::map_entity_to_item(m))
            .transpose()?)
    }

    async fn find_items_by_todo_id(&self, todo_id: i64) -> Result<Vec<TodoItem>, anyhow::Error> {
        let models = todo_item_entity::Entity::find()
            .filter(todo_item_entity::Column::TodoId.eq(todo_id))
            .all(&self.db)
            .await
            .map_err(DatabaseError::QueryError)?;

        let items = models
            .into_iter()
            .map(TodoMapper::map_entity_to_item)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(items)
    }

    async fn insert_item(
        &self,
        item: &TodoItem,
        parent_todo_id: i64,
    ) -> Result<TodoItem, anyhow::Error> {
        let mut model = TodoMapper::map_item_to_entity(item);
        model.todo_id = Set(parent_todo_id);

        let saved = model
            .insert(&self.db)
            .await
            .map_err(DatabaseError::QueryError)?;

        TodoMapper::map_entity_to_item(saved)
    }

    async fn update_item(&self, item: &TodoItem) -> Result<TodoItem, anyhow::Error> {
        let active = TodoMapper::map_item_to_entity(item);

        let saved = active
            .update(&self.db)
            .await
            .map_err(DatabaseError::QueryError)?;

        TodoMapper::map_entity_to_item(saved)
    }

    async fn delete_item(&self, item_id: i64) -> Result<(), anyhow::Error> {
        todo_item_entity::Entity::delete_by_id(item_id)
            .exec(&self.db)
            .await
            .map_err(DatabaseError::QueryError)?;

        Ok(())
    }
}
