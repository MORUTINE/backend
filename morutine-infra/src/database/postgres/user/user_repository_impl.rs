use anyhow::Error;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use domain::user::models::user::User;
use domain::user::repository::user_repository::UserRepository;

use crate::database::database_error_code::DatabaseErrorCode;
use crate::database::postgres::user::user_entity;
use crate::database::postgres::user::user_mapper::UserMapper;

pub struct UserRepositoryImpl {
    pub db: sea_orm::DatabaseConnection,
}

impl UserRepositoryImpl {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, Error> {
        let user = user_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(DatabaseErrorCode::QueryError)?;

        Ok(user.map(|u| UserMapper::map_to_model(u)).transpose()?)
    }

    async fn find_by_nickname(&self, nickname: &str) -> Result<Option<User>, Error> {
        let user = user_entity::Entity::find()
            .filter(user_entity::Column::Nickname.eq(nickname))
            .one(&self.db)
            .await
            .map_err(DatabaseErrorCode::QueryError)?;

        Ok(user.map(|u| UserMapper::map_to_model(u)).transpose()?)
    }

    async fn insert(&self, user: &mut User) -> Result<User, Error> {
        let user = UserMapper::map_to_entity(user);

        let saved = user
            .insert(&self.db)
            .await
            .map_err(DatabaseErrorCode::QueryError)?;

        Ok(UserMapper::map_to_model(saved)?)
    }

    async fn update(&self, user: &User) -> Result<User, Error> {
        let user = UserMapper::map_to_entity(user);

        let updated = user
            .update(&self.db)
            .await
            .map_err(DatabaseErrorCode::QueryError)?;

        Ok(UserMapper::map_to_model(updated)?)
    }

    async fn delete(&self, id: i64) -> Result<(), Error> {
        user_entity::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(DatabaseErrorCode::QueryError)?;

        Ok(())
    }
}
