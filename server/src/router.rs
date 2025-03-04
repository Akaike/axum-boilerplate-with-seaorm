use crate::common::state::AppState;
use crate::todo;

use axum::Router;

pub fn init() -> Router<AppState> {
    Router::new().nest("/api/v1/todos", todo::router::init())
}
