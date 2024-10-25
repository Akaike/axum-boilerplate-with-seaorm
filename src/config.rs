use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_audience: String,
    pub jwt_issuer: String,
    pub jwks_uri: String,
    pub log_level: String,
}

pub fn load_config() -> Config {
    envy::from_env::<Config>().expect("Failed to load configuration")
}
