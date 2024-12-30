use chrono::Utc;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use super::jwks;
use crate::{config::CONFIG, error::AuthError};

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
    let jwks = jwks::fetch(&CONFIG.jwks_uri).await?;
    let jwk = jwks
        .find(kid)
        .ok_or_else(|| AuthError::InvalidToken("Key not found in JWKS".into()))?;

    DecodingKey::from_jwk(jwk)
        .map_err(|e| AuthError::InvalidToken(format!("Failed to create decoding key: {}", e)))
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
    let current_time = Utc::now().timestamp();
    if claims.exp <= current_time {
        Err(AuthError::TokenExpired)
    } else {
        Ok(())
    }
}
