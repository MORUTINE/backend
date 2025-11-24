use common::constant::status::BAD_REQUEST;
use common::error::{ErrorCode, ErrorReason};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserErrorCode {
    #[error("유효하지 않은 닉네임")]
    InvalidNickname,
    #[error("유효하지 않은 OAuth 제공자")]
    InvalidProvider,
    #[error("유효하지 않은 OAuth 유저 ID")]
    InvalidProviderUserId,
}

impl ErrorCode for UserErrorCode {
    fn reason(&self) -> ErrorReason {
        match self {
            UserErrorCode::InvalidNickname => ErrorReason {
                status: BAD_REQUEST,
                code: "USER_400_1",
                message: "유효하지 않은 닉네임입니다.",
            },
            UserErrorCode::InvalidProvider => ErrorReason {
                status: BAD_REQUEST,
                code: "USER_400_2",
                message: "유효하지 않은 OAuth 제공자입니다.",
            },
            UserErrorCode::InvalidProviderUserId => ErrorReason {
                status: BAD_REQUEST,
                code: "USER_400_3",
                message: "유효하지 않은 OAuth 유저 ID입니다.",
            },
        }
    }
}
