use axum::{
    routing::{get, post},
    Router,
};

use crate::{handler::todo, state::app::AppState};

pub fn init() -> Router<AppState> {
    Router::new()
        .route("/:todo_id", get(todo::get_by_id).put(todo::update))
        .route("/", post(todo::create))
    // middleware jwt auth example
    //.route_layer(middleware::from_fn(is_authenticated))
}