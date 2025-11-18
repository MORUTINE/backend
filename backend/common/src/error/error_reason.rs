use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ErrorReason {
    pub status: u16,
    pub code: &'static str,
    pub message: &'static str,
}