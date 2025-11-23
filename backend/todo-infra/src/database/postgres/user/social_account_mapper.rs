use crate::database::postgres::user::social_account_entity;
use domain::user::models::social_account::SocialAccountBuilder;
use domain::user::{OAuthProvider, SocialAccount};
use sea_orm::{NotSet, Set};
use std::str::FromStr;

pub struct SocialAccountMapper;

impl SocialAccountMapper {
    pub fn map_to_entity(model: &SocialAccount) -> social_account_entity::ActiveModel {
        social_account_entity::ActiveModel {
            id: model.id().map(Set).unwrap_or(NotSet),
            user_id: Set(model.user_id()),
            provider: Set(model.provider().as_str().to_string()),
            provider_user_id: Set(model.provider_user_id().to_string()),
            created_at: Set(model.created_at().into()),
            modified_at: Set(model.modified_at().into()),
        }
    }

    pub fn map_to_model(
        entity: social_account_entity::Model,
    ) -> Result<SocialAccount, anyhow::Error> {
        let provider = OAuthProvider::from_str(entity.provider.as_str())?;

        Ok(SocialAccountBuilder::default()
            .id(Some(entity.id))
            .user_id(entity.user_id)
            .provider(provider)
            .provider_user_id(entity.provider_user_id)
            .created_at(entity.created_at.into())
            .modified_at(entity.modified_at.into())
            .build()?)
    }
}
