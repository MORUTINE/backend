// use axum::extract::{Query, State};
// use axum::response::IntoResponse;
// use serde::Deserialize;
// use domain::todo::todo_error_code::TodoErrorCode;
// use crate::bootstrap::AppState;
// use crate::common::error::app_error::AppError;
// use crate::common::response::api_response::ApiResponse;
//
// #[derive(Deserialize)]
// pub struct MonthlyQuery {
//     pub year: i32,
//     pub month: u32,
// }
//
// pub async fn get_monthly_todos(
//     State(state): State(AppState),
//     Query(q): Query<MonthlyQuery>,
// ) -> Result<ApiResponse<T>, AppError<E>> {
//
// }
