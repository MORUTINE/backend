use crate::user::error::UserError;
use crate::user::models::oauth_provider::OAuthProvider;
use chrono::{DateTime, Utc};
use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct SocialAccount {
    id: Option<i64>,
    user_id: i64,
    provider: OAuthProvider,
    provider_user_id: String,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

impl SocialAccount {
    pub fn new(
        user_id: i64,
        provider: OAuthProvider,
        provider_user_id: String,
    ) -> Result<Self, UserError> {
        if provider_user_id.trim().is_empty() {
            return Err(UserError::InvalidProviderUserId);
        }

        let now = Utc::now();

        Ok(Self {
            id: None,
            user_id,
            provider,
            provider_user_id,
            created_at: now,
            modified_at: now,
        })
    }

    pub fn id(&self) -> Option<i64> {
        self.id
    }
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
    pub fn provider(&self) -> &OAuthProvider {
        &self.provider
    }
    pub fn provider_user_id(&self) -> &str {
        &self.provider_user_id
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn modified_at(&self) -> DateTime<Utc> {
        self.modified_at
    }
}
