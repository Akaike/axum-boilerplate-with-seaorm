#![allow(non_upper_case_globals)]

use crate::error::{ApiError, Error};
use cached::proc_macro::cached;

#[cached(
    name = "fetch_cache",
    time = 3600,
    key = "String",
    convert = r#"{ url.to_string() }"#,
    result = true
)]
pub async fn fetch_json_cached(url: &str) -> Result<serde_json::Value, Error> {
    let res = reqwest::get(url)
        .await
        .map_err(|e| {
            Error::Api(ApiError::RequestFailed(format!(
                "Failed to fetch resource: {}",
                e
            )))
        })?
        .error_for_status()
        .map_err(|e| Error::Api(ApiError::RequestFailed(format!("Request failed: {}", e))))?;

    let body = res.text().await.map_err(|e| {
        Error::Api(ApiError::RequestFailed(format!(
            "Failed to get response body: {}",
            e
        )))
    })?;

    serde_json::from_str(&body).map_err(|e| {
        Error::Api(ApiError::ParseError(format!(
            "Failed to parse response: {}. Body: {}",
            e, body
        )))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_fetch_success() {
        let mock_server = MockServer::start().await;
        let test_json = json!({
            "keys": [{
                "kty": "RSA",
                "kid": "test-key-id",
                "n": "test-n",
                "e": "AQAB"
            }]
        });

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&test_json))
            .mount(&mock_server)
            .await;

        let result = fetch_json_cached(&mock_server.uri()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_json);

        // Test caching - second call should use cached value
        let second_result = fetch_json_cached(&mock_server.uri()).await;
        assert!(second_result.is_ok());
        assert_eq!(second_result.unwrap(), test_json);
    }

    #[tokio::test]
    async fn test_fetch_invalid_json() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string("invalid json"))
            .mount(&mock_server)
            .await;

        let result = fetch_json_cached(&mock_server.uri()).await;
        assert!(matches!(
            result.unwrap_err(),
            Error::Api(ApiError::ParseError(_))
        ));
    }

    #[tokio::test]
    async fn test_fetch_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let result = fetch_json_cached(&mock_server.uri()).await;
        assert!(matches!(
            result.unwrap_err(),
            Error::Api(ApiError::RequestFailed(_))
        ));
    }

    #[tokio::test]
    async fn test_fetch_network_error() {
        let result = fetch_json_cached("http://invalid-url").await;
        assert!(matches!(
            result.unwrap_err(),
            Error::Api(ApiError::RequestFailed(_))
        ));
    }
}
