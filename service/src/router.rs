use std::sync::Arc;

use crate::{app::AppState, routes};
use axum::Router;

pub fn init() -> Router<Arc<AppState>> {
    Router::new().merge(routes::todo::init())
}
