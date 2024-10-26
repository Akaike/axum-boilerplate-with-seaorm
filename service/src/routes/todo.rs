use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{app::AppState, handler::todo};

pub fn init() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/todos/:todo_id", get(todo::get_by_id))
        .route("/api/todos", post(todo::create))
}
