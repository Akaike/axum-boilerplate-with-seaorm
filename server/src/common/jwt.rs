use chrono::Utc;
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use super::fetch;
use crate::config::CONFIG;
use crate::common::error::ApiError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub async fn validate(token: &str) -> Result<Claims, ApiError> {
    let kid = extract_kid(token)?;
    let decoding_key = extract_decoding_key(&kid).await?;
    let validation = create_validation();
    let claims = validate_and_get_claims(token, &decoding_key, &validation)?;

    Ok(claims)
}

fn extract_kid(token: &str) -> Result<String, ApiError> {
    let header = decode_header(token)
        .map_err(|e| ApiError::Unauthorized(format!("Invalid token header: {}", e)))?;
    header
        .kid
        .ok_or_else(|| ApiError::Unauthorized("Missing KID in token".into()))
}

async fn extract_decoding_key(kid: &str) -> Result<DecodingKey, ApiError> {
    let jwks = fetch_jwks().await?;

    let jwk = jwks
        .find(kid)
        .ok_or_else(|| ApiError::Unauthorized("Key not found in JWKS".into()))?;

    DecodingKey::from_jwk(jwk)
        .map_err(|e| ApiError::Unauthorized(format!("Failed to create decoding key: {}", e)))
}

async fn fetch_jwks() -> Result<JwkSet, ApiError> {
    let json_value = fetch::fetch_json_cached(&CONFIG.jwks_uri)
        .await
        .map_err(|e| ApiError::Unauthorized(format!("Failed to fetch JWKS: {}", e)))?;

    serde_json::from_value(json_value)
        .map_err(|e| ApiError::Unauthorized(format!("Failed to parse JWKS: {}", e)))
}

fn create_validation() -> Validation {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[&CONFIG.jwt_audience]);
    validation.set_issuer(&[&CONFIG.jwt_issuer]);
    validation
}

fn validate_and_get_claims(
    token: &str,
    decoding_key: &DecodingKey,
    validation: &Validation,
) -> Result<Claims, ApiError> {
    let token_data = decode::<Claims>(token, decoding_key, validation)
        .map_err(|err| ApiError::Unauthorized(format!("Token validation failed: {}", err)))?;

    validate_expiration(&token_data.claims)?;
    
    Ok(token_data.claims)
}

fn validate_expiration(claims: &Claims) -> Result<(), ApiError> {
    match claims.exp <= Utc::now().timestamp() {
        true => Err(ApiError::Unauthorized("Token expired".to_string())),
        false => Ok(()),
    }
}