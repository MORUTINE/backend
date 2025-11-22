use crate::user::error::UserError;
use UserError::InvalidProvider;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OAuthProvider {
    Kakao,
}

impl OAuthProvider {
    pub fn from_str(provider: &str) -> Result<Self, UserError> {
        match provider.to_lowercase().as_str() {
            "kakao" => Ok(OAuthProvider::Kakao),
            _ => Err(InvalidProvider),
        }
    }
}
