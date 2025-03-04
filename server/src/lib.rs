pub mod config;
pub mod database;
pub mod middleware {
    pub mod auth;
}
pub mod router;
pub mod server;
pub mod todo {
    pub mod model;
    pub mod repository;
    pub mod service;
    pub mod controller;
    pub mod router;
}
pub mod common {
    pub mod error;
    pub mod state;
    pub mod fetch;
    pub mod jwt;
    pub mod validated_json;
}

use std::error::Error;

use common::state::AppState;
use config::CONFIG;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
pub async fn start() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(CONFIG.log_level.clone())
        .init();

    let db = database::create(&CONFIG).await;
    let app_state = AppState::new(db);

    let router = router::init()
        .with_state(app_state)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));
    let server = server::create(router).await?;
    let signal = tokio::signal::ctrl_c();

    tokio::select! {
        res = server => {
            if let Err(err) = res {
                tracing::error!("Server error: {}", err);
            }
        }
        _ = signal => {
            tracing::info!("Shutdown signal received");
        }
    }

    Ok(())
}

pub fn main() {
    if let Err(err) = start() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
