use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{handler::todo, middleware::auth::is_authenticated, state::app::AppState};

pub fn init() -> Router<AppState> {
    let router = Router::new()
        .route("/:todo_id", get(todo::get_by_id).put(todo::update))
        .route("/", post(todo::create));

    // Uncomment to enable JWT authentication for all routes in this router
    // router.layer(middleware::from_fn(is_authenticated));

    router
}
