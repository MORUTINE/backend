use chrono::Datelike;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct GetMonthlyTodosRequest {
    pub year: u32,
    pub month: u32,
}

impl Default for GetMonthlyTodosRequest {
    fn default() -> Self {
        let now = chrono::Utc::now().naive_utc().date();
        Self {
            year: Datelike::year(&now) as u32,
            month: Datelike::month(&now),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AddItemRequest {
    pub date: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItemContentRequest {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItemStatusRequest {
    pub altered_content: Option<String>,
}
