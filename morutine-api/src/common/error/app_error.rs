#[derive(Debug)]
pub struct AppError(pub anyhow::Error);

impl AppError {
    pub fn new(e: impl Into<anyhow::Error>) -> Self {
        Self(e.into())
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
