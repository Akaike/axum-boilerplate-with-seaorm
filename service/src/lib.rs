pub mod app;
pub mod config;
pub mod database;
pub mod dto;
pub mod error;
pub mod handler;
pub mod repository;
pub mod router;
pub mod routes;
pub mod server;
pub mod service;

use std::error::Error;

use app::AppState;
use config::load_config;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
pub async fn start() -> Result<(), Box<dyn Error>> {
    let config = load_config();

    tracing_subscriber::fmt()
        .with_env_filter(config.log_level.clone())
        .init();

    let db = database::postgres::create(&config).await;
    let app_state = AppState::new(config, db);

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
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
