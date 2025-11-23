use crate::database::error::DatabaseError;
use crate::database::postgres::user::social_account_entity;
use crate::database::postgres::user::social_account_mapper::SocialAccountMapper;
use anyhow::Error;
use async_trait::async_trait;
use domain::user::{SocialAccount, SocialAccountRepository};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

pub struct SocialAccountRepositoryImpl {
    pub db: sea_orm::DatabaseConnection,
}

impl SocialAccountRepositoryImpl {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SocialAccountRepository for SocialAccountRepositoryImpl {
    async fn find_by_provider_and_user_id(
        &self,
        provider: domain::user::OAuthProvider,
        provider_user_id: &str,
    ) -> Result<Option<SocialAccount>, anyhow::Error> {
        let social_account = social_account_entity::Entity::find()
            .filter(social_account_entity::Column::Provider.eq(provider.as_str().to_string()))
            .filter(social_account_entity::Column::ProviderUserId.eq(provider_user_id))
            .one(&self.db)
            .await
            .map_err(DatabaseError::QueryError)?;

        Ok(social_account
            .map(|s| SocialAccountMapper::map_to_model(s))
            .transpose()?)
    }

    async fn find_by_user_id_and_provider(
        &self,
        user_id: i64,
        provider: &domain::user::OAuthProvider,
    ) -> Result<Option<SocialAccount>, anyhow::Error> {
        let social_account = social_account_entity::Entity::find()
            .filter(social_account_entity::Column::UserId.eq(user_id))
            .filter(social_account_entity::Column::Provider.eq(provider.as_str().to_string()))
            .one(&self.db)
            .await
            .map_err(DatabaseError::QueryError)?;

        Ok(social_account
            .map(|s| SocialAccountMapper::map_to_model(s))
            .transpose()?)
    }

    async fn save(&self, social_account: &SocialAccount) -> Result<SocialAccount, Error> {
        let social_account = SocialAccountMapper::map_to_entity(social_account);

        let saved = social_account
            .insert(&self.db)
            .await
            .map_err(DatabaseError::QueryError)?;

        Ok(SocialAccountMapper::map_to_model(saved)?)
    }
}
