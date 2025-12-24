use crate::bootstrap::AppState;
use crate::common::auth::UserId;
use crate::common::error::app_error::AppError;
use crate::domain::todo::{GetMonthlyTodosRequest, MonthlyTodoResponse};
use axum::{
    Extension, Json,
    extract::{Query, State},
    response::IntoResponse,
};
use http::StatusCode;

pub async fn get_monthly_todos(
    State(state): State<AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
    Query(query): Query<GetMonthlyTodosRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result = state
        .todo_service
        .get_monthly_todos(user_id, query.year, query.month)
        .await?;

    Ok((StatusCode::OK, Json(MonthlyTodoResponse::try_from(result)?)))
}
