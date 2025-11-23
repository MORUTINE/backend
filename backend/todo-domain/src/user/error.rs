use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("유효하지 않은 닉네임")]
    InvalidNickname,
    #[error("유효하지 않은 OAuth 제공자")]
    InvalidProvider,
    #[error("유효하지 않은 OAuth 유저 ID")]
    InvalidProviderUserId,
}
