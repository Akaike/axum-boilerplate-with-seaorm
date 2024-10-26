use axum::{
    routing::{get, post},
    Router,
};

use crate::{app::AppState, handler::todo};

pub fn init() -> Router<AppState> {
    Router::new()
        .route("/:todo_id", get(todo::get_by_id))
        .route("/", post(todo::create))
}
