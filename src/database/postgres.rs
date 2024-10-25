use std::time::Duration;

use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::Config;

pub async fn create_pool(config: &Config) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(5000)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Some(Duration::from_secs(1800)))
        .connect(&config.database_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create postgres database pool: {}", e))
}
