mod todo;

use crate::state::app::AppState;
use axum::Router;

pub fn init() -> Router<AppState> {
    Router::new().nest("/api/v1/todos", todo::init())
}
