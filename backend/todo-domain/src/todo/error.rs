use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoError {
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
