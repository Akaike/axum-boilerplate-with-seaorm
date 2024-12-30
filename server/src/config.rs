use dotenvy::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub log_level: String,
    pub jwks_uri: String,
    pub jwt_audience: String,
    pub jwt_issuer: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();
    envy::from_env::<Config>().expect("Failed to load configuration")
});
