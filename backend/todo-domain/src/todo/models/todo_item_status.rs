#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TodoItemStatus {
    Pending,   // 아직 수행 전
    Completed, // 정상 완료
    Altered,   // 대체 업무 수행
    Failed,    // 실패
}
