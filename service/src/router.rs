use crate::{app::AppState, routes};
use axum::Router;

pub fn init() -> Router<AppState> {
    Router::new().merge(routes::todo::init())
}
