use super::error_reason::ErrorReason;

pub trait ErrorCode {
    fn reason(&self) -> ErrorReason;
}
