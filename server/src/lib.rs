pub mod config;
pub mod database;
pub mod dto;
pub mod error;
pub mod handler;
pub mod middleware;
pub mod repository;
pub mod router;
pub mod server;
pub mod service;
pub mod state;
pub mod util;
pub mod validator;

use std::error::Error;

use config::CONFIG;
use state::app::AppState;
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
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
