use super::{todo_entity, todo_item_entity};
use domain::todo::models::todo::TodoFromEntity;
use domain::todo::models::todo_item::TodoItemFromEntity;
use domain::todo::models::todo_item_status::TodoItemStatus;
use domain::todo::models::{todo::Todo, todo_item::TodoItem};
use sea_orm::{NotSet, Set};

pub struct TodoMapper;

impl TodoMapper {
    pub fn map_todo_to_entity(todo: &Todo) -> todo_entity::ActiveModel {
        todo_entity::ActiveModel {
            id: todo.id().map(Set).unwrap_or(NotSet),
            user_id: Set(todo.user_id()),
            date: Set(todo.date()),
            created_at: Set(todo.created_at().into()),
            modified_at: Set(todo.modified_at().into()),
        }
    }

    pub fn map_item_to_entity(item: &TodoItem) -> todo_item_entity::ActiveModel {
        todo_item_entity::ActiveModel {
            id: item.id().map(Set).unwrap_or(NotSet),
            todo_id: Set(item.todo_id()),
            content: Set(item.content().to_string()),
            status: Set(item.status().as_str().to_string()),
            altered_content: Set(item.altered_plan().cloned()),
            image_url: Set(item.image_url().cloned()),
            created_at: Set(item.created_at().into()),
            modified_at: Set(item.modified_at().into()),
        }
    }

    pub fn map_entity_to_todo(
        todo: todo_entity::Model,
        items: Vec<todo_item_entity::Model>,
    ) -> Result<Todo, anyhow::Error> {
        let from_entity = Self::map_entity_to_todo_from_entity(todo, items)?;
        Ok(Todo::from_entity(from_entity))
    }

    pub fn map_entity_to_item(entity: todo_item_entity::Model) -> Result<TodoItem, anyhow::Error> {
        let from_entity = Self::map_entity_to_item_from_entity(entity)?;
        Ok(TodoItem::from_entity(from_entity))
    }

    pub fn map_entity_to_todo_from_entity(
        todo: todo_entity::Model,
        items: Vec<todo_item_entity::Model>,
    ) -> Result<TodoFromEntity, anyhow::Error> {
        let item_from_entities = items
            .into_iter()
            .map(|e| Self::map_entity_to_item_from_entity(e))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TodoFromEntity {
            id: todo.id,
            user_id: todo.user_id,
            date: todo.date,
            items: item_from_entities,
            created_at: todo.created_at.into(),
            modified_at: todo.modified_at.into(),
        })
    }

    pub fn map_entity_to_item_from_entity(
        entity: todo_item_entity::Model,
    ) -> Result<TodoItemFromEntity, anyhow::Error> {
        let status = TodoItemStatus::try_from(entity.status.as_str())?;

        Ok(TodoItemFromEntity {
            id: entity.id,
            todo_id: entity.todo_id,
            content: entity.content,
            status,
            altered_content: entity.altered_content,
            image_url: entity.image_url,
            created_at: entity.created_at.into(),
            modified_at: entity.modified_at.into(),
        })
    }
}
