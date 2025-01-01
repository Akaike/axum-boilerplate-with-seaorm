use chrono::Utc;
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use super::fetch;
use crate::{
    config::CONFIG,
    error::{ApiError, AuthError, Error},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub async fn validate(token: &str) -> Result<Claims, AuthError> {
    let kid = extract_kid(token)?;
    let decoding_key = extract_decoding_key(&kid).await?;
    let validation = create_validation();
    let claims = validate_and_get_claims(token, &decoding_key, &validation)?;

    Ok(claims)
}

fn extract_kid(token: &str) -> Result<String, AuthError> {
    let header = decode_header(token)
        .map_err(|e| AuthError::InvalidToken(format!("Invalid token header: {}", e)))?;
    header
        .kid
        .ok_or_else(|| AuthError::InvalidToken("Missing KID in token".into()))
}

async fn extract_decoding_key(kid: &str) -> Result<DecodingKey, AuthError> {
    let jwks = fetch_jwks().await?;

    let jwk = jwks
        .find(kid)
        .ok_or_else(|| AuthError::InvalidToken("Key not found in JWKS".into()))?;

    DecodingKey::from_jwk(jwk)
        .map_err(|e| AuthError::InvalidToken(format!("Failed to create decoding key: {}", e)))
}

async fn fetch_jwks() -> Result<JwkSet, AuthError> {
    let json_value = fetch::fetch_json_cached(&CONFIG.jwks_uri)
        .await
        .map_err(|e| match e {
            Error::Api(ApiError::RequestFailed(msg)) => {
                AuthError::JwksError(format!("Failed to fetch JWKS: {}", msg))
            }
            Error::Api(ApiError::ParseError(msg)) => {
                AuthError::JwksError(format!("Invalid JWKS response: {}", msg))
            }
            _ => AuthError::JwksError("Unexpected error fetching JWKS".to_string()),
        })?;

    serde_json::from_value(json_value)
        .map_err(|e| AuthError::JwksError(format!("Failed to parse JWKS: {}", e)))
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
) -> Result<Claims, AuthError> {
    let token_data = decode::<Claims>(token, decoding_key, validation)
        .map_err(|err| AuthError::InvalidToken(format!("Token validation failed: {}", err)))?;

    validate_expiration(&token_data.claims)?;
    
    Ok(token_data.claims)
}

fn validate_expiration(claims: &Claims) -> Result<(), AuthError> {
    match claims.exp <= Utc::now().timestamp() {
        true => Err(AuthError::TokenExpired),
        false => Ok(()),
    }
}