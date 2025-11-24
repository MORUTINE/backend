use common::error::ErrorCode;

#[derive(Debug)]
pub struct AppError<E: ErrorCode> {
    pub code: E,
}

impl<E: ErrorCode> AppError<E> {
    pub fn new(code: E) -> Self {
        Self { code }
    }
}
