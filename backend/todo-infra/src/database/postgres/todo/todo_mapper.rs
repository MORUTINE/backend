use super::{todo_entity, todo_item_entity};
use domain::todo::todo_error_code::TodoErrorCode;
use domain::todo::models::{todo::Todo, todo_item::TodoItem};
use domain::todo::models::{
    todo::TodoBuilder, todo_item::TodoItemBuilder, todo_item_status::TodoItemStatus,
};
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
    ) -> Result<Todo, TodoErrorCode> {
        let mut item_models = Vec::new();

        for i in items {
            let status = TodoItemStatus::try_from(i.status.as_str())?;

            let item = TodoItemBuilder::default()
                .id(Some(i.id))
                .todo_id(i.todo_id)
                .content(i.content)
                .status(status)
                .altered_content(i.altered_content)
                .image_url(i.image_url)
                .created_at(i.created_at.into())
                .modified_at(i.modified_at.into())
                .build()
                .map_err(|_| TodoErrorCode::InvalidStatus)?;

            item_models.push(item);
        }

        let todo_model = TodoBuilder::default()
            .id(Some(todo.id))
            .user_id(todo.user_id)
            .date(todo.date)
            .items(item_models)
            .created_at(todo.created_at.into())
            .modified_at(todo.modified_at.into())
            .build()
            .map_err(|_| TodoErrorCode::InvalidStatus)?;

        Ok(todo_model)
    }

    pub fn map_entity_to_item(entity: todo_item_entity::Model) -> Result<TodoItem, anyhow::Error> {
        let status = TodoItemStatus::try_from(entity.status.as_str())?;

        TodoItemBuilder::default()
            .id(Some(entity.id))
            .todo_id(entity.todo_id)
            .content(entity.content)
            .status(status)
            .altered_content(entity.altered_content)
            .image_url(entity.image_url)
            .created_at(entity.created_at.into())
            .modified_at(entity.modified_at.into())
            .build()
            .map_err(Into::into)
    }
}
