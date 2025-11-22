#[derive(Debug)]
pub enum TodoError {
    MaxItemLimit,
    EmptyContent,
    PastDateNotAllowed,
    ItemNotFound,
    StateChangeNotAllowed,
}
