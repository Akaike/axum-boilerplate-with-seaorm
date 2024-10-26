use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub log_level: String,
}

pub fn load_config() -> Config {
    dotenv().ok();

    envy::from_env::<Config>().expect("Failed to load configuration")
}
