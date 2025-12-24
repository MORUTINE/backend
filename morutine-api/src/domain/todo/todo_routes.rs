use axum::Router;
use axum::routing::{delete, get, patch, post};

use crate::bootstrap::AppState;
use crate::domain::todo::presentation::todo_handlers::get_monthly_todos;
// use crate::domain::todo::presentation::todo_handlers::{
//     create_todo_item, delete_todo_item, get_todo, update_item_status, update_todo_item,
// };

pub fn todo_routes() -> Router<AppState> {
    Router::new().route("/", get(get_monthly_todos))
    // .route("/:date/items", post(create_todo_item))
    // .route(
    //     "/:date/items/:item_id",
    //     patch(update_todo_item).delete(delete_todo_item),
    // )
    // .route("/:date/items/:item_id/status/:status", patch(update_item_status))
}
