use axum::{extract::Request, middleware::Next, response::Response};
use reqwest::header;

use crate::{common::error::ApiError, common::jwt};

pub async fn is_authenticated(mut req: Request, next: Next) -> Result<Response, ApiError> {
    let token = extract_token(&req)?;
    let claims = jwt::validate(token).await?;

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}

fn extract_token(req: &Request) -> Result<&str, ApiError> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_header| auth_header.strip_prefix("Bearer "))
        .ok_or(ApiError::Unauthorized("Missing token"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{HeaderMap, Request},
    };
    use reqwest::header;

    fn create_request_with_headers(headers: HeaderMap) -> Request<Body> {
        let mut req = Request::builder()
            .uri("http://example.com/test")
            .body(Body::empty())
            .unwrap();

        *req.headers_mut() = headers;
        req
    }

    #[test]
    fn test_extract_token_valid_bearer() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            "Bearer valid_token_123".try_into().unwrap(),
        );

        let req = create_request_with_headers(headers);
        let result = extract_token(&req);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "valid_token_123");
    }

    #[test]
    fn test_extract_token_missing_header() {
        let headers = HeaderMap::new();
        let req = create_request_with_headers(headers);

        let result = extract_token(&req);
        assert!(result.is_err());

        assert!(matches!(
            result,
            Err(ApiError::Unauthorized("Missing token"))
        ));
    }

    #[test]
    fn test_extract_token_invalid_format() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            "InvalidFormat token123".try_into().unwrap(),
        );

        let req = create_request_with_headers(headers);
        let result = extract_token(&req);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ApiError::Unauthorized("Missing token"))
        ));
    }

    #[test]
    fn test_extract_token_bearer_no_token() {
        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, "Bearer".try_into().unwrap());

        let req = create_request_with_headers(headers);
        let result = extract_token(&req);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ApiError::Unauthorized("Missing token"))
        ));
    }

    #[test]
    fn test_extract_token_bearer_empty_token() {
        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, "Bearer ".try_into().unwrap());

        let req = create_request_with_headers(headers);
        let result = extract_token(&req);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_extract_token_invalid_utf8() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            axum::http::HeaderValue::from_bytes(&[0xFF, 0xFE]).unwrap(),
        );

        let req = create_request_with_headers(headers);
        let result = extract_token(&req);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ApiError::Unauthorized("Missing token"))
        ));
    }
}
