use crate::user::error::UserError;
use chrono::{DateTime, Utc};
use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct User {
    id: Option<i64>,
    nickname: String,
    profile_image_url: Option<String>,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

impl User {
    pub fn new(nickname: String, profile_image_url: Option<String>) -> Result<Self, UserError> {
        if nickname.trim().is_empty() {
            return Err(UserError::InvalidNickname);
        }

        let now = Utc::now();

        Ok(User {
            id: None,
            nickname,
            profile_image_url,
            created_at: now,
            modified_at: now,
        })
    }

    pub fn id(&self) -> Option<i64> {
        self.id
    }
    pub fn nickname(&self) -> &str {
        &self.nickname
    }
    pub fn profile_image_url(&self) -> Option<&String> {
        self.profile_image_url.as_ref()
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn modified_at(&self) -> DateTime<Utc> {
        self.modified_at
    }
}
