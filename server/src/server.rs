use std::error::Error;

use axum::{serve::Serve, Router};
use tokio::net::TcpListener;
use tracing::info;

pub async fn create(router: Router) -> Result<Serve<TcpListener, Router, Router>, Box<dyn Error>> {
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    let address = tcp_listener.local_addr()?;

    info!("Listening on {}", address);

    Ok(axum::serve(tcp_listener, router))
}
