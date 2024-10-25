use anyhow::Error;
use core_rs_rewrite::{config::load_config, database};

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    let config = load_config();

    let database = database::postgres::create_pool(&config).await?;

    let database_manager = database::manager::DatabaseManagerImpl::new(database);

    let signal = tokio::signal::ctrl_c();
    tokio::select! {
        _ = signal => {
            tracing::info!("Shutdown signal received");
        }
    }

    Ok(())
}
