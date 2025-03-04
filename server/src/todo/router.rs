use axum::{
    routing::{get, post},
    Router,
};

use crate::{todo::controller, common::state::AppState};

pub fn init() -> Router<AppState> {
    let router = Router::new()
        .route(
            "/{todo_id}",
            get(controller::get_by_id).put(controller::update).delete(controller::delete),
        )
        .route("/", post(controller::create));

    // Uncomment to enable JWT authentication for all routes in this router
    // router.layer(middleware::from_fn(is_authenticated));

    router
}
