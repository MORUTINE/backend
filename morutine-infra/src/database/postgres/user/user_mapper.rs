use super::user_entity;
use domain::user::models::user::{User, UserBuilder};
use sea_orm::{NotSet, Set};

pub struct UserMapper;

impl UserMapper {
    pub fn map_to_entity(model: &User) -> user_entity::ActiveModel {
        user_entity::ActiveModel {
            id: model.id().map(Set).unwrap_or(NotSet),
            nickname: Set(model.nickname().to_string()),
            profile_image_url: Set(model.profile_image_url().cloned()),
            created_at: Set(model.created_at().into()),
            modified_at: Set(model.modified_at().into()),
        }
    }

    pub fn map_to_model(entity: user_entity::Model) -> Result<User, anyhow::Error> {
        Ok(UserBuilder::default()
            .id(Some(entity.id))
            .nickname(entity.nickname)
            .profile_image_url(entity.profile_image_url)
            .created_at(entity.created_at.into())
            .modified_at(entity.modified_at.into())
            .build()?)
    }
}
