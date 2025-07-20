use chrono::Utc;
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tracing::debug;

use super::fetch;
use crate::common::error::ApiError;
use crate::config::CONFIG;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub async fn validate(token: &str) -> Result<Claims, ApiError> {
    let kid = extract_kid(token)?;
    let decoding_key = extract_decoding_key(&kid).await?;
    let validation = build_validation();
    let claims = decode_and_verify(token, &decoding_key, &validation)?;

    Ok(claims)
}

fn extract_kid(token: &str) -> Result<String, ApiError> {
    let header = decode_header(token).map_err(|e| {
        debug!(error = ?e, "invalid JWT header");
        ApiError::Unauthorized("invalid token")
    })?;

    header.kid.ok_or(ApiError::Unauthorized("invalid token"))
}

async fn extract_decoding_key(kid: &str) -> Result<DecodingKey, ApiError> {
    let jwks = fetch_jwks().await?;
    let jwk = jwks
        .find(kid)
        .ok_or(ApiError::Unauthorized("invalid token"))?;

    DecodingKey::from_jwk(jwk).map_err(|e| {
        debug!(error = ?e, "failed to build decoding key");
        ApiError::Unauthorized("invalid token")
    })
}

async fn fetch_jwks() -> Result<JwkSet, ApiError> {
    let json_val = fetch::fetch_json_cached(&CONFIG.jwks_uri)
        .await
        .map_err(|e| {
            debug!(error = ?e, "JWKS fetch failed");
            ApiError::Unauthorized("invalid token")
        })?;

    serde_json::from_value(json_val).map_err(|e| {
        debug!(error = ?e, "JWKS parse failed");
        ApiError::Unauthorized("invalid token")
    })
}

fn build_validation() -> Validation {
    let mut v = Validation::new(Algorithm::RS256);
    v.set_audience(&[&CONFIG.jwt_audience]);
    v.set_issuer(&[&CONFIG.jwt_issuer]);
    v
}

fn decode_and_verify(
    token: &str,
    key: &DecodingKey,
    validation: &Validation,
) -> Result<Claims, ApiError> {
    let data = decode::<Claims>(token, key, validation).map_err(|e| {
        debug!(error = ?e, "JWT signature/claims invalid");
        ApiError::Unauthorized("invalid token")
    })?;

    verify_exp(&data.claims)?;
    Ok(data.claims)
}

fn verify_exp(claims: &Claims) -> Result<(), ApiError> {
    if claims.exp <= Utc::now().timestamp() {
        Err(ApiError::Unauthorized("token expired"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use chrono::{Duration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};

    #[test]
    fn test_claims_serialization() {
        let claims = Claims {
            sub: "user123".to_string(),
            exp: Utc::now().timestamp() + 3600,
        };

        let json = serde_json::to_string(&claims).unwrap();
        let deserialized: Claims = serde_json::from_str(&json).unwrap();

        assert_eq!(claims.sub, deserialized.sub);
        assert_eq!(claims.exp, deserialized.exp);
    }

    #[test]
    fn test_verify_exp_valid_token() {
        let future_exp = Utc::now() + Duration::hours(1);
        let claims = Claims {
            sub: "user123".to_string(),
            exp: future_exp.timestamp(),
        };

        assert!(verify_exp(&claims).is_ok());
    }

    #[test]
    fn test_verify_exp_expired_token() {
        let past_exp = Utc::now() - Duration::hours(1);
        let claims = Claims {
            sub: "user123".to_string(),
            exp: past_exp.timestamp(),
        };

        let result = verify_exp(&claims);
        assert!(result.is_err());

        assert!(matches!(
            result,
            Err(ApiError::Unauthorized("token expired"))
        ));
    }

    #[test]
    fn test_extract_kid_missing_kid() {
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let claims = Claims {
            sub: "user123".to_string(),
            exp: Utc::now().timestamp() + 3600,
        };

        let key = EncodingKey::from_secret(b"secret");
        let token = encode(&header, &claims, &key).unwrap();

        let result = extract_kid(&token);
        assert!(result.is_err());

        assert!(matches!(
            result,
            Err(ApiError::Unauthorized("invalid token"))
        ));
    }

    #[test]
    fn test_extract_kid_with_kid() {
        let mut header = Header::new(jsonwebtoken::Algorithm::HS256);
        header.kid = Some("test-key-id".to_string());

        let claims = Claims {
            sub: "user123".to_string(),
            exp: Utc::now().timestamp() + 3600,
        };

        let key = EncodingKey::from_secret(b"secret");
        let token = encode(&header, &claims, &key).unwrap();

        let result = extract_kid(&token);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-key-id");
    }

    #[test]
    fn test_extract_kid_invalid_token() {
        let invalid_token = "invalid.jwt.token";
        let result = extract_kid(invalid_token);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ApiError::Unauthorized("invalid token"))
        ));
    }

    #[test]
    fn test_build_validation_structure() {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&["test-audience"]);
        validation.set_issuer(&["test-issuer"]);

        assert_eq!(validation.algorithms, vec![Algorithm::RS256]);
        assert!(validation.aud.is_some());
        assert!(validation.iss.is_some());

        let mut expected_aud = HashSet::new();
        expected_aud.insert("test-audience".to_string());
        assert_eq!(validation.aud, Some(expected_aud));

        let mut expected_iss = HashSet::new();
        expected_iss.insert("test-issuer".to_string());
        assert_eq!(validation.iss, Some(expected_iss));
    }

    #[test]
    fn test_decode_and_verify_invalid_signature() {
        let mut header = Header::new(jsonwebtoken::Algorithm::HS256);
        header.kid = Some("test-key-id".to_string());

        let claims = Claims {
            sub: "user123".to_string(),
            exp: Utc::now().timestamp() + 3600,
        };

        let key1 = EncodingKey::from_secret(b"secret1");
        let token = encode(&header, &claims, &key1).unwrap();

        let key2 = DecodingKey::from_secret(b"secret2");
        let validation = Validation::new(Algorithm::HS256);

        let result = decode_and_verify(&token, &key2, &validation);
        assert!(result.is_err());

        assert!(matches!(
            result,
            Err(ApiError::Unauthorized("invalid token"))
        ));
    }
}
