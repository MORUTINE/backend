#[derive(Debug)]
pub struct AppError<E: super::error_code::ErrorCode> {
    pub code: E,
}

impl<E: super::error_code::ErrorCode> AppError<E> {
    pub fn new(code: E) -> Self {
        Self { code }
    }
}