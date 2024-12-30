use sea_orm::{Database, DatabaseConnection};

use crate::config::Config;

pub async fn create(config: &Config) -> DatabaseConnection {
    Database::connect(&config.database_url)
        .await
        .expect("Failed to connect to the database")
}
