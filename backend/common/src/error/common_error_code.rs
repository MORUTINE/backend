use super::{error_code::ErrorCode, error_reason::ErrorReason};
use crate::constant::status::*;

#[derive(Debug)]
pub enum CommonErrorCode {
    InvalidParam,
    Unauthorized,
    Forbidden,
    NotFound,
    TimeOut,
    Conflict,
    TooManyRequests,
    InternalServerError,
}

impl ErrorCode for CommonErrorCode {
    fn reason(&self) -> ErrorReason {
        match self {
            CommonErrorCode::InvalidParam => ErrorReason {
                status: BAD_REQUEST,
                code: "COMMON_400_INVALID_PARAM",
                message: "요청 파라미터가 올바르지 않습니다.",
            },
            CommonErrorCode::Unauthorized => ErrorReason {
                status: UNAUTHORIZED,
                code: "COMMON_401_UNAUTHORIZED",
                message: "인증이 필요합니다.",
            },
            CommonErrorCode::Forbidden => ErrorReason {
                status: FORBIDDEN,
                code: "COMMON_403_FORBIDDEN",
                message: "접근 권한이 없습니다.",
            },
            CommonErrorCode::NotFound => ErrorReason {
                status: NOT_FOUND,
                code: "COMMON_404_NOT_FOUND",
                message: "요청한 리소스를 찾을 수 없습니다.",
            },
            CommonErrorCode::TimeOut => ErrorReason {
                status: TIME_OUT,
                code: "COMMON_408_TIME_OUT",
                message: "요청을 처리하는 데 시간이 초과되었습니다.",
            },
            CommonErrorCode::Conflict => ErrorReason {
                status: CONFLICT,
                code: "COMMON_409_CONFLICT",
                message: "요청 처리 중에 충돌이 발생했습니다.",
            },
            CommonErrorCode::TooManyRequests => ErrorReason {
                status: TOO_MANY_REQUESTS,
                code: "COMMON_429_TOO_MANY_REQUESTS",
                message: "요청이 너무 많습니다. 잠시후 재시도 해주세요.",
            },
            CommonErrorCode::InternalServerError => ErrorReason {
                status: INTERNAL_SERVER_ERROR,
                code: "COMMON_500_INTERNAL_SERVER_ERROR",
                message: "서버 내부에 오류가 발생했습니다.",
            },
        }
    }
}
