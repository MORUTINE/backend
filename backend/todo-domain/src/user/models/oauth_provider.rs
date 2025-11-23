use std::str::FromStr;
use crate::user::error::UserError;
use UserError::InvalidProvider;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OAuthProvider {
    Kakao,
}

impl OAuthProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            OAuthProvider::Kakao => "KAKAO",
        }
    }
}

impl FromStr for OAuthProvider {
    type Err = UserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kakao" => Ok(OAuthProvider::Kakao),
            _ => Err(InvalidProvider),
        }
    }
}
