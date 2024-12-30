#![allow(non_upper_case_globals)]

use crate::error::AuthError;
use cached::proc_macro::cached;
use jsonwebtoken::jwk::JwkSet;

pub async fn fetch(jwks_uri: &str) -> Result<JwkSet, AuthError> {
    jwks_cache_prime_cache(jwks_uri).await
}

#[cached(
    name = "jwks_cache",
    time = 3600,
    key = "String",
    convert = r#"{ jwks_uri.to_string() }"#,
    result = true
)]
async fn jwks_cache_prime_cache(jwks_uri: &str) -> Result<JwkSet, AuthError> {
    fetch_from_uri(jwks_uri).await
}

async fn fetch_from_uri(jwks_uri: &str) -> Result<JwkSet, AuthError> {
    let res = reqwest::get(jwks_uri)
        .await
        .map_err(|e| AuthError::InvalidToken(format!("Failed to fetch JWKS: {}", e)))?
        .error_for_status()
        .map_err(|e| AuthError::InvalidToken(format!("JWKS request failed: {}", e)))?;

    let body = res
        .text()
        .await
        .map_err(|e| AuthError::InvalidToken(format!("Failed to get JWKS response: {}", e)))?;

    serde_json::from_str(&body).map_err(|e| {
        AuthError::InvalidToken(format!("Failed to parse JWKS: {}. Body: {}", e, body))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_fetch_success() {
        let mock_server = MockServer::start().await;
        let jwks_json = r#"{
            "keys": [{
                "kty": "RSA",
                "kid": "test-key-id",
                "n": "test-n",
                "e": "AQAB"
            }]
        }"#;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string(jwks_json))
            .mount(&mock_server)
            .await;

        let result = fetch(&mock_server.uri()).await;
        assert!(result.is_ok());

        // Test caching - second call should use cached value
        let second_result = fetch(&mock_server.uri()).await;
        assert!(second_result.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_invalid_json() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string("invalid json"))
            .mount(&mock_server)
            .await;

        let result = fetch(&mock_server.uri()).await;
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(matches!(err, AuthError::InvalidToken(_)));
    }

    #[tokio::test]
    async fn test_fetch_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let result = fetch(&mock_server.uri()).await;
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(matches!(err, AuthError::InvalidToken(_)));
    }
}
