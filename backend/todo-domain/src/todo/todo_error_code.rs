use thiserror::Error;
use common::constant::status::{BAD_REQUEST, CONFLICT, FORBIDDEN, NOT_FOUND};
use common::error::{ErrorCode, ErrorReason};

#[derive(Debug, Error)]
pub enum TodoErrorCode {
    #[error("하루 할 일 제한(3개) 초과")]
    MaxItemLimit,
    #[error("내용이 비어 있을 수 없음")]
    EmptyContent,
    #[error("과거 날짜엔 할 일 추가/변경/삭제 불가능")]
    PastDateNotAllowed,
    #[error("해당 할 일을 찾을 수 없음")]
    ItemNotFound,
    #[error("상태를 변경할 수 없음")]
    StateChangeNotAllowed,
    #[error("유효하지 않은 상태값")]
    InvalidStatus,
}

impl ErrorCode for TodoErrorCode {
    fn reason(&self) -> ErrorReason {
        match self {
            TodoErrorCode::MaxItemLimit => ErrorReason {
                status: CONFLICT,
                code: "TODO_409_1",
                message: "하루 최대 3개까지 등록 가능합니다.",
            },
            TodoErrorCode::EmptyContent => ErrorReason {
                status: BAD_REQUEST,
                code: "TODO_400_1",
                message: "내용이 비어 있을 수 없습니다.",
            },
            TodoErrorCode::PastDateNotAllowed => ErrorReason {
                status: FORBIDDEN,
                code: "TODO_403_1",
                message: "과거 날짜에는 작업할 수 없습니다.",
            },
            TodoErrorCode::ItemNotFound => ErrorReason {
                status: NOT_FOUND,
                code: "TODO_404_1",
                message: "등록된 할 일을 찾을 수 없습니다.",
            },
            TodoErrorCode::StateChangeNotAllowed => ErrorReason {
                status: FORBIDDEN,
                code: "TODO_403_2",
                message: "해당 할 일의 상태 변경이 불가능합니다.",
            },
            TodoErrorCode::InvalidStatus => ErrorReason {
                status: BAD_REQUEST,
                code: "TODO_400_2",
                message: "유효하지 않은 상태값입니다.",
            },
        }
    }
}

